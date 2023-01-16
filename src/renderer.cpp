#include <WaveSabreCore.h>
#include <WaveSabrePlayerLib.h>

using namespace WaveSabreCore;
using namespace WaveSabrePlayerLib;

extern "C"
{
    double wsc_song_length(const unsigned char *indata)
    {
        return *(double *)(indata + sizeof(int) * 2);
    }

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
