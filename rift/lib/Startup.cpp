
namespace rift {

void Startup(int argc, char *argv[]) {}

} // namespace rift

extern "C" {
static void rift_startup(int argc, char *argv[]) { return rift::Startup(argc, argv); }
}
