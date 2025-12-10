use std::{fmt::Debug, time::Duration};

use tokio::{spawn, task::JoinHandle, time::sleep};

use crate::{
    BotOperationUpdate, Settings,
    ecs::{Resources, World},
    navigator::Navigator,
    operation::Operation,
    player::{Panic, PanicTo, PlayerAction},
    rotator::Rotator,
    services::Event,
};

const PENDING_HALT_SECS: u64 = 12;

pub enum OperationEvent {
    Halt,
}

impl Event for OperationEvent {}

/// A service to handle operation-related incoming requests.
pub trait OperationService: Debug {
    /// Polls for any pending [`OperationEvent`].
    fn poll(&mut self, navigator: &dyn Navigator) -> Option<OperationEvent>;

    /// Applies the provided `update` to other arguments.
    fn apply(
        &mut self,
        resources: &mut Resources,
        world: &mut World,
        rotator: &mut dyn Rotator,
        settings: &Settings,
        update: BotOperationUpdate,
    );

    /// Halts the bot and optionally go to town.
    fn halt(
        &mut self,
        resources: &mut Resources,
        world: &mut World,
        rotator: &mut dyn Rotator,
        go_to_town: bool,
    );

    /// Queues a halt that results in a [`OperationEvent::Halt`] when the timer ends.
    fn queue_halt(&mut self);
}

#[derive(Debug, Default)]
pub struct DefaultOperationService {
    pending_halt: Option<JoinHandle<()>>,
}

impl DefaultOperationService {
    fn clear_states(&mut self, world: &mut World, rotator: &mut dyn Rotator, should_idle: bool) {
        rotator.reset_queue();
        world.player.context.clear_actions_aborted(should_idle);
        if let Some(handle) = self.pending_halt.take() {
            handle.abort();
        }
    }
}

impl OperationService for DefaultOperationService {
    fn poll(&mut self, navigator: &dyn Navigator) -> Option<OperationEvent> {
        if self
            .pending_halt
            .as_ref()
            .is_some_and(|handle| handle.is_finished())
        {
            self.pending_halt = None;
            if !navigator.was_last_point_available_or_completed() {
                return Some(OperationEvent::Halt);
            }
        }

        None
    }

    fn apply(
        &mut self,
        resources: &mut Resources,
        world: &mut World,
        rotator: &mut dyn Rotator,
        settings: &Settings,
        update: BotOperationUpdate,
    ) {
        let cycle_run_stop = settings.cycle_run_stop;
        let cycle_run_duration_millis = settings.cycle_run_duration_millis;
        let cycle_stop_duration_millis = settings.cycle_stop_duration_millis;

        let operation = resources.operation;
        resources.operation = operation.update_from_bot_update_and_mode(
            update,
            cycle_run_stop,
            cycle_run_duration_millis,
            cycle_stop_duration_millis,
        );

        if matches!(
            update,
            BotOperationUpdate::Halt | BotOperationUpdate::TemporaryHalt
        ) {
            self.clear_states(world, rotator, true);
        }
    }

    fn halt(
        &mut self,
        resources: &mut Resources,
        world: &mut World,
        rotator: &mut dyn Rotator,
        go_to_town: bool,
    ) {
        self.clear_states(world, rotator, !go_to_town);

        if !resources.operation.halting() {
            resources.operation = Operation::Halting;
        }

        if go_to_town {
            rotator.inject_action(PlayerAction::Panic(Panic { to: PanicTo::Town }));
        }
    }

    fn queue_halt(&mut self) {
        self.pending_halt = Some(spawn(async move {
            sleep(Duration::from_secs(PENDING_HALT_SECS)).await;
        }));
    }
}
