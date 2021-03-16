List<String> wordList = [
  'aeioulnrst',
  'dg',
  'bcmp',
  'fhvwy',
  'k',
  '',
  '',
  'jx',
  '',
  'qz',
];

int score(String s) {
  int out = 0;
  for(String c in s.split('')) {
    for(int i = 0; i < wordList.length; i++) {
      if(wordList[i].contains(c.toLowerCase())) {
        out += i + 1;
        break;
      }
    }
  }
  return out;
}