extern "C" void Reset();
extern "C" void DefaultHandler();
using ExceptionHandler = void (*)();

static ExceptionHandler reset_vector __attribute__ ((section (".vector_table.reset_vector"))) = Reset;

static ExceptionHandler exceptions[14] __attribute__ ((section (".vector_table.exceptions"))) = {
    DefaultHandler, DefaultHandler, DefaultHandler, DefaultHandler, DefaultHandler,
    nullptr, nullptr, nullptr, nullptr,
    DefaultHandler, nullptr, nullptr, DefaultHandler, DefaultHandler
};

static ExceptionHandler interrupts[240] __attribute__ ((section (".vector_table.interrupts"))) = { DefaultHandler };