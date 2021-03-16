class ResistorColor {
  List<String> get colors => [
        'black',
        'brown',
        'red',
        'orange',
        'yellow',
        'green',
        'blue',
        'violet',
        'grey',
        'white',
      ];

  int colorCode(String s) {
    return this.colors.indexOf(s);
  }
}
