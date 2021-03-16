class ResistorColorDuo {
  List<String> get resistors => [
        "black",
        "brown",
        "red",
        "orange",
        "yellow",
        "green",
        "blue",
        "violet",
        "grey",
        "white"
      ];
  int value(List<String> list) {
    int sum = 0;

    sum += this.resistors.indexOf(list[0]) * 10;
    sum += this.resistors.indexOf(list[1]);

    return sum;
  }
}
