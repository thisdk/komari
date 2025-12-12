use std::fmt::Debug;

use tokio::sync::broadcast::Receiver;

use super::EventContext;
use crate::{
    BotOperationUpdate,
    ecs::WorldEvent,
    notification::NotificationKind,
    player::{PanicTo, Panicking, Player},
    services::EventHandler,
};

/// A service to handle world-related incoming requests.
pub trait WorldService: Debug {
    /// Polls for any pending [`WorldEvent`].
    fn poll(&mut self) -> Option<WorldEvent>;
}

#[derive(Debug)]
pub struct DefaultWorldService {
    event_rx: Receiver<WorldEvent>,
}

impl DefaultWorldService {
    pub fn new(event_rx: Receiver<WorldEvent>) -> Self {
        Self { event_rx }
    }
}

impl WorldService for DefaultWorldService {
    fn poll(&mut self) -> Option<WorldEvent> {
        self.event_rx.try_recv().ok()
    }
}

pub struct WorldEventHandler;

impl EventHandler<WorldEvent> for WorldEventHandler {
    fn handle(&mut self, context: &mut EventContext<'_>, event: WorldEvent) {
        match event {
            WorldEvent::CycledToHalt => {
                context.operation_service.halt(
                    context.resources,
                    context.world,
                    context.rotator,
                    true,
                );
            }
            WorldEvent::PlayerDied => {
                if context.settings_service.settings().stop_on_player_die {
                    context.operation_service.halt(
                        context.resources,
                        context.world,
                        context.rotator,
                        false,
                    );
                }
            }
            WorldEvent::MinimapChanged => {
                if context.resources.operation.halting() {
                    return;
                }

                let _ = context
                    .resources
                    .notification
                    .schedule_notification(NotificationKind::FailOrMapChange);

                if !context
                    .settings_service
                    .settings()
                    .stop_on_fail_or_change_map
                {
                    return;
                }

                let is_panicking = matches!(
                    context.world.player.state,
                    Player::Panicking(Panicking {
                        to: PanicTo::Channel,
                        ..
                    })
                );
                if is_panicking {
                    return;
                }

                context.operation_service.queue_halt();
            }
            WorldEvent::CaptureFailed => {
                if context.resources.operation.halting() {
                    return;
                }

                if context
                    .settings_service
                    .settings()
                    .stop_on_fail_or_change_map
                {
                    context.operation_service.apply(
                        context.resources,
                        context.world,
                        context.rotator,
                        &context.settings_service.settings(),
                        BotOperationUpdate::TemporaryHalt,
                    );
                }
                let _ = context
                    .resources
                    .notification
                    .schedule_notification(NotificationKind::FailOrMapChange);
            }
            WorldEvent::LieDetectorAppeared => {
                let _ = context
                    .resources
                    .notification
                    .schedule_notification(NotificationKind::LieDetectorAppear);
            }
        }
    }
}
