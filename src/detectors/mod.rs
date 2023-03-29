use self::detector::Detector;

pub mod controlled_library_call;
pub mod dead_code;
pub mod detector;
pub mod unused_arguments;
pub mod unused_events;

pub fn get_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::<controlled_library_call::ControlledLibraryCall>::default(),
        Box::<unused_events::UnusedEvents>::default(),
        Box::<dead_code::DeadCode>::default(),
        Box::<unused_arguments::UnusedArguments>::default(),
    ]
}
