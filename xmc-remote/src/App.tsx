import { useEffect, useState } from 'react';
import './App.css';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import Toolbar from '@mui/material/Toolbar';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import { Source, Power, Mode, XmcStatus, getStatus, volumeUp, volumeDown, setVolume, setMode, setSource, powerOn, standBy } from './services/api'
import { SelectChangeEvent } from '@mui/material/Select';
import StatusCard from './components/StatusCard';
import VolumeCard from './components/VolumeCard';
import AppBar from './components/AppBar';
import SourceCard from './components/SourceCard';

const mdTheme = createTheme();

function App() {
  const [xmcStatus, setXmcStatus] = useState<XmcStatus>({ power: Power.Off, source: Source.HDMI1, volume: -100, mode: Mode.surround, audioBits: "", audioBitstream: "" });
  useEffect(() => {
    setInterval(() => {
      getStatus().then(setXmcStatus)
    }, 2000)
  }, [])
  const onPowerToggle = () => {
    if (xmcStatus.power === Power.On) {
      setXmcStatus((prev: XmcStatus) => ({ ...prev, power: Power.Off }))
      standBy()
    }
    else if (xmcStatus.power === Power.Off) {
      setXmcStatus((prev: XmcStatus) => ({ ...prev, power: Power.On }))
      powerOn()
    }
  }

  const onVolumeChange = (volume: number) => {
    setXmcStatus((prev: XmcStatus) => ({ ...prev, volume }))
    setVolume(volume)
  }

  const onModeChange = (e: SelectChangeEvent) => {
    const mode = e.target.value as Mode
    setXmcStatus((prev: XmcStatus) => ({ ...prev, mode }))
    setMode(mode)
  }

  const onInputChange = (e: SelectChangeEvent) => {
    const source = e.target.value as Source
    setXmcStatus((prev: XmcStatus) => ({ ...prev, source }))
    setSource(source)
  }

  return (
    <ThemeProvider theme={mdTheme}>
      <Box sx={{ display: 'flex' }}>
        <CssBaseline />
        <AppBar />
        <Box
          component="main"
          sx={{
            backgroundColor: (theme) =>
              theme.palette.mode === 'light'
                ? theme.palette.grey[100]
                : theme.palette.grey[900],
            flexGrow: 1,
            height: '100vh',
            overflow: 'auto',
          }}
        >
          <Toolbar />
          <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
            <Grid container spacing={3}>
              <Grid item xs={12} md={8} lg={9}>
                <StatusCard
                  audioInfo={xmcStatus.audioBits}
                  audioBitstream={xmcStatus.audioBitstream}
                  onPowerToggle={onPowerToggle}
                  power={xmcStatus.power === Power.On}
                />
              </Grid>
              <Grid item xs={12} md={8} lg={9}>
                <SourceCard
                  onModeChange={onModeChange}
                  mode={xmcStatus.mode}
                  onInputChange={onInputChange}
                  source={xmcStatus.source}
                />
              </Grid>
              <Grid item xs={12} md={4} lg={3}>
                <VolumeCard
                  volumeDown={volumeDown}
                  volumeUp={volumeUp}
                  onVolumeChange={onVolumeChange}
                  volume={xmcStatus.volume}
                />
              </Grid>
            </Grid>
          </Container>
        </Box>
      </Box>

    </ThemeProvider >
  );
}

export default App;
