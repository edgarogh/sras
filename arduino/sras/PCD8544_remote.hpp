class RemPCD8544 {
  public:
  void begin(int width, int height);
  void createChar(char c, unsigned char* glyph);
  void write(char c);
  void print(int num, int radix);
  void setCursor(char x, char y);
  void clear(void);
};
