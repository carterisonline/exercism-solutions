class BeerSong {
  List<String> recite(int i, int j) {
    List<String> out = [];
    for(int ii = i; ii > i - j; ii--) {
      String num = '$ii';
      if(ii == 0) {
        num = 'no more';
      }
      if(ii == 1) {
        out.add('1 bottle of beer on the wall, 1 bottle of beer.');
      } else {
        out.add('${num.replaceFirst('n', 'N')} bottles of beer on the wall, $num bottles of beer.');
      }
      if(num == 'no more') {
        out.add('Go to the store and buy some more, 99 bottles of beer on the wall.');
      } else if(ii == 2) {
        out.add('Take one down and pass it around, 1 bottle of beer on the wall.');
      } else if(ii == 1) {
        out.add('Take it down and pass it around, no more bottles of beer on the wall.');
      } else {
        out.add('Take one down and pass it around, ${ii-1} bottles of beer on the wall.');
      }

      if(ii != i - j + 1) {
        out.add('');
      }
    }
    return out;
  }
}
