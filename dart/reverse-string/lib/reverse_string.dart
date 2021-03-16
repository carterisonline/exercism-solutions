String reverse(String s) {
  String out = '';
  for(String i in s.split('')) {
    out = "$i$out";
  }
  return out;
}
