#include <WaveSabreCore.h>
#include <WaveSabrePlayerLib.h>

using namespace WaveSabreCore;
using namespace WaveSabrePlayerLib;

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

    int wsc_render(Device *(*factory)(SongRenderer::DeviceId), const unsigned char *indata, unsigned short *samples)
    {
        SongRenderer::Song song;
        song.blob = indata;
        song.factory = factory;

        SYSTEM_INFO sysinfo;
        GetSystemInfo(&sysinfo);

        SongRenderer songRenderer(&song, sysinfo.dwNumberOfProcessors);
        const auto sampleRate = songRenderer.GetSampleRate();
        const int stepSize = 100 * SongRenderer::NumChannels;
        const auto renderBufferSize = (int)((double)(sampleRate * SongRenderer::NumChannels) * songRenderer.GetLength()) / stepSize * stepSize;
        const auto renderBuffer = new SongRenderer::Sample[renderBufferSize];
        for (int i = 0; i < renderBufferSize; i += stepSize)
        {
            songRenderer.RenderSamples(renderBuffer + i, stepSize);
        }

        samples = reinterpret_cast<unsigned short *>(renderBuffer);
        return renderBufferSize;
    }

    IPlayer *wsc_play(Device *(*factory)(SongRenderer::DeviceId), const unsigned char *indata)
    {
        SongRenderer::Song song;
        song.blob = indata;
        song.factory = factory;

        SYSTEM_INFO sysinfo;
        GetSystemInfo(&sysinfo);

        auto player = new PreRenderPlayer(&song, sysinfo.dwNumberOfProcessors, nullptr, nullptr);
        player->Play();
        return player;
    }

    void wsc_free(IPlayer *player)
    {
        delete player;
    }
}
