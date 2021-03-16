class Leap {
  bool leapYear(int i) {
    if(i % 4 == 0) {
      if(i % 100 != 0 || i % 400 == 0) {
        return true;
      }
    }

    return false;
  }
}
