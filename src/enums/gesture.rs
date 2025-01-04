pub enum Gesture {
  GestureNone = 0,        // No gesture
  GestureTap = 1,         // Tap gesture
  GestureDoubletap = 2,   // Double tap gesture
  GestureHold = 4,        // Hold gesture
  GestureDrag = 8,        // Drag gesture
  GestureSwipeRight = 16, // Swipe right gesture
  GestureSwipeLeft = 32,  // Swipe left gesture
  GestureSwipeUp = 64,    // Swipe up gesture
  GestureSwipeDown = 128, // Swipe down gesture
  GesturePinchIn = 256,   // Pinch in gesture
  GesturePinchOut = 512,  // Pinch out gesture
}
