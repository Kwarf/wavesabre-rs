#include <WaveSabreCore.h>

using namespace WaveSabreCore;

extern "C"
{
    Device *falcon() { return new WaveSabreCore::Falcon(); }
    Device *slaughter() { return new WaveSabreCore::Slaughter(); }
    Device *thunder() { return new WaveSabreCore::Thunder(); }
    Device *scissor() { return new WaveSabreCore::Scissor(); }
    Device *leveller() { return new WaveSabreCore::Leveller(); }
    Device *crusher() { return new WaveSabreCore::Crusher(); }
    Device *echo() { return new WaveSabreCore::Echo(); }
    Device *smasher() { return new WaveSabreCore::Smasher(); }
    Device *chamber() { return new WaveSabreCore::Chamber(); }
    Device *twister() { return new WaveSabreCore::Twister(); }
    Device *cathedral() { return new WaveSabreCore::Cathedral(); }
    Device *adultery() { return new WaveSabreCore::Adultery(); }
    Device *specimen() { return new WaveSabreCore::Specimen(); }
}
