//! Gesture event types

use super::EventTrait;
use {AsRaw, FromRaw};
use ffi;

/// Common functions all Gesture-Events implement.
pub trait GestureEventTrait: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(
    /// The event time for this event
    fn time, ffi::libinput_event_gesture_get_time, u32);
    ffi_func!(
    /// The event time for this event in microseconds
    fn time_usec, ffi::libinput_event_gesture_get_time_usec, u64);
    ffi_func!(
    /// Return the number of fingers used for a gesture.
    ///
    /// This can be used e.g. to differentiate between 3 or 4 finger swipes.
    ///
    /// This function can be called on all gesture events and the returned finger
    /// count value will not change during a sequence.
    fn finger_count, ffi::libinput_event_gesture_get_finger_count, i32);

    /// Convert into a general `GestureEvent` again
    fn into_gesture_event(self) -> GestureEvent
        where Self: Sized
    {
        unsafe { GestureEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_gesture>> GestureEventTrait for T {}

/// A gesture related `Event`
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GestureEvent {
    /// A swipe gesture `Event`
    Swipe(GestureSwipeEvent),
    /// A pinch gesture `Event`
    Pinch(GesturePinchEvent),
}

impl EventTrait for GestureEvent {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GestureEvent::Swipe(ref event) => event.as_raw_event(),
            GestureEvent::Pinch(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_gesture> for GestureEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        let base = ffi::libinput_event_gesture_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END => {
                GestureEvent::Swipe(GestureSwipeEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END => {
                GestureEvent::Pinch(GesturePinchEvent::from_raw(event))
            }
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_gesture> for GestureEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GestureEvent::Swipe(ref event) => event.as_raw(),
            GestureEvent::Pinch(ref event) => event.as_raw(),
        }
    }
}

/// Common functions for Gesture-Events having coordinates.
pub trait GestureEventCoordinates: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(
    /// Return the delta between the last event and the current event.
    ///
    /// If a device employs pointer acceleration, the delta returned by this
    /// function is the accelerated delta.
    ///
    /// Relative motion deltas are normalized to represent those of a device with
    /// 1000dpi resolution. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details.
    fn dx, ffi::libinput_event_gesture_get_dx, f64);
    ffi_func!(
    /// Return the relative delta of the unaccelerated motion vector of the
    /// current event.
    ///
    /// Relative unaccelerated motion deltas are normalized to represent those of
    /// a device with 1000dpi resolution. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details. Note that unaccelerated events are not equivalent to
    /// 'raw' events as read from the device.
    ///
    /// Any rotation applied to the device also applies to gesture motion (see
    /// `rotation_set_angle`).
    fn dx_unaccelerated, ffi::libinput_event_gesture_get_dx_unaccelerated, f64);
    ffi_func!(
    /// Return the delta between the last event and the current event.
    ///
    /// If a device employs pointer acceleration, the delta returned by this
    /// function is the accelerated delta.
    ///
    /// Relative motion deltas are normalized to represent those of a device with
    /// 1000dpi resolution. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details.
    fn dy, ffi::libinput_event_gesture_get_dy, f64);
    ffi_func!(
    /// Return the relative delta of the unaccelerated motion vector of the
    /// current event.
    ///
    /// Relative unaccelerated motion deltas are normalized to represent those of
    /// a device with 1000dpi resolution. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details. Note that unaccelerated events are not equivalent to
    /// 'raw' events as read from the device.
    ///
    /// Any rotation applied to the device also applies to gesture motion (see
    /// `rotation_set_angle`).
    fn dy_unaccelerated, ffi::libinput_event_gesture_get_dy_unaccelerated, f64);
}

/// Common functions for events noting the end of a gesture
pub trait GestureEndEvent: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(
    /// Return if the gesture ended normally, or if it was cancelled.
    fn cancelled, ffi::libinput_event_gesture_get_cancelled, bool);
}

/// Events for swipe gestures
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GestureSwipeEvent {
    /// Swipe gesture began
    Begin(GestureSwipeBeginEvent),
    /// In-progress swipe gesture updated
    Update(GestureSwipeUpdateEvent),
    /// Swipe gesture ended
    End(GestureSwipeEndEvent),
}

/// Common functions for swipe gesture events
pub trait GestureSwipeEventTrait: AsRaw<ffi::libinput_event_gesture> {
    /// Convert into a general `GestureSwipeEvent`
    fn into_gesture_swipe_event(self) -> GestureSwipeEvent
        where Self: Sized
    {
        unsafe { GestureSwipeEvent::from_raw(self.as_raw_mut()) }
    }
}

impl GestureSwipeEventTrait for GestureSwipeEvent {}

impl EventTrait for GestureSwipeEvent {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GestureSwipeEvent::Begin(ref event) => event.as_raw_event(),
            GestureSwipeEvent::Update(ref event) => event.as_raw_event(),
            GestureSwipeEvent::End(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_gesture> for GestureSwipeEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        let base = ffi::libinput_event_gesture_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN => {
                GestureSwipeEvent::Begin(GestureSwipeBeginEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE => {
                GestureSwipeEvent::Update(GestureSwipeUpdateEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END => {
                GestureSwipeEvent::End(GestureSwipeEndEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END => panic!("Tried to make GestureSwipeEvent from Pinch event"),
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_gesture> for GestureSwipeEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GestureSwipeEvent::Begin(ref event) => event.as_raw(),
            GestureSwipeEvent::Update(ref event) => event.as_raw(),
            GestureSwipeEvent::End(ref event) => event.as_raw(),
        }
    }
}

/// Events for pinch gestures
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GesturePinchEvent {
    /// Pinch gesture began
    Begin(GesturePinchBeginEvent),
    /// In-progress pinch gesture updated
    Update(GesturePinchUpdateEvent),
    /// Pinch gesture ended
    End(GesturePinchEndEvent),
}

impl EventTrait for GesturePinchEvent {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GesturePinchEvent::Begin(ref event) => event.as_raw_event(),
            GesturePinchEvent::Update(ref event) => event.as_raw_event(),
            GesturePinchEvent::End(ref event) => event.as_raw_event(),
        }
    }
}

/// Common functions for pinch gesture events
pub trait GesturePinchEventTrait: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(
    /// Return the absolute scale of a pinch gesture, the scale is the division of
    /// the current distance between the fingers and the distance at the start of
    /// the gesture.
    ///
    /// The scale begins at 1.0, and if e.g. the fingers moved together by 50%
    /// then the scale will become 0.5, if they move twice as far apart as
    /// initially the scale becomes 2.0, etc.
    ///
    /// For gesture events that are of type `GesturePinchBeginEvent`, this function
    /// returns 1.0.
    /// For gesture events that are of type `GesturePinchEndEvent`, this function
    /// returns the scale value of the most recent `GesturePinchUpdateEvent` event
    /// (if any) or 1.0 otherwise.
    fn scale, ffi::libinput_event_gesture_get_scale, f64);

    /// Convert into a general `GesturePinchEvent`
    fn into_gesture_pinch_event(self) -> GesturePinchEvent
        where Self: Sized
    {
        unsafe { GesturePinchEvent::from_raw(self.as_raw_mut()) }
    }
}

impl GesturePinchEventTrait for GesturePinchEvent {}

impl FromRaw<ffi::libinput_event_gesture> for GesturePinchEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        let base = ffi::libinput_event_gesture_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE |
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END => panic!("Tried to make GesturePinchEvent from Swipe event"),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN => {
                GesturePinchEvent::Begin(GesturePinchBeginEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE => {
                GesturePinchEvent::Update(GesturePinchUpdateEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END => {
                GesturePinchEvent::End(GesturePinchEndEvent::from_raw(event))
            }
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_gesture> for GesturePinchEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GesturePinchEvent::Begin(ref event) => event.as_raw(),
            GesturePinchEvent::Update(ref event) => event.as_raw(),
            GesturePinchEvent::End(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(
/// Swipe gesture began
struct GestureSwipeBeginEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GestureSwipeEventTrait for GestureSwipeBeginEvent {}

ffi_event_struct!(
/// In-progress swipe gesture updated
struct GestureSwipeUpdateEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GestureSwipeEventTrait for GestureSwipeUpdateEvent {}

impl GestureEventCoordinates for GestureSwipeUpdateEvent {}

ffi_event_struct!(
/// Swipe gesture ended
struct GestureSwipeEndEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GestureEndEvent for GestureSwipeEndEvent {}

impl GestureSwipeEventTrait for GestureSwipeEndEvent {}

ffi_event_struct!(
/// Pinch gesture began
struct GesturePinchBeginEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GesturePinchEventTrait for GesturePinchBeginEvent {}

ffi_event_struct!(
/// In-progress pinch gesture updated
struct GesturePinchUpdateEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GesturePinchEventTrait for GesturePinchUpdateEvent {}

impl GestureEventCoordinates for GesturePinchUpdateEvent {}

impl GesturePinchUpdateEvent {
    ffi_func!(
    /// Return the angle delta in degrees between the last and the current
    /// `GesturePinchUpdateEvent`.
    ///
    /// The angle delta is defined as the change in angle of the line formed by
    /// the 2 fingers of a pinch gesture. Clockwise rotation is represented by a
    /// positive delta, counter-clockwise by a negative delta. If e.g. the fingers
    /// are on the 12 and 6 location of a clock face plate and they move to the 1
    /// resp. 7 location in a single event then the angle delta is 30 degrees.
    ///
    /// If more than two fingers are present, the angle represents the rotation
    /// around the center of gravity. The calculation of the center of gravity is
    /// implementation-dependent.
    pub fn angle_delta, ffi::libinput_event_gesture_get_angle_delta, f64);
}

ffi_event_struct!(
/// Pinch gesture ended
struct GesturePinchEndEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GesturePinchEventTrait for GesturePinchEndEvent {}

impl GestureEndEvent for GesturePinchEndEvent {}
