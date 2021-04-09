typedef unsigned char byte;

namespace screen {
  enum class SetupStatus : char {
    PENDING = ' ',
    OK = 1,
  };

  class SetupTask {
    public:
    byte index = 0;
    void set_status(SetupStatus status);
  };

  class Setup {
    public:
    byte loc = 1;
    Setup();
    ~Setup();
    SetupTask add_task(const char* name);
  };

  class UI {
    private:
    long next_update;

    public:
    UI();

    void loop(void);

    char lightness = 0;
  };

  void init(void);
  Setup init_setup(void);
  void loop(void);
}
