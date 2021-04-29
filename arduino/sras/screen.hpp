typedef unsigned char byte;

namespace screen {
  class UI {
    private:
    long next_update;

    public:
    UI();

    void loop(void);

    char lightness = 0;
  };

  void init(void);
  void loop(void);
}
