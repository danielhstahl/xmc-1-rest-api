import { useCallback, useEffect, useRef, useState } from 'react';
import './App.css';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import Toolbar from '@mui/material/Toolbar';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import { Source, Power, XmcReadOnly, XmcWrite, getReadOnly, getWrite, volumeUp, volumeDown, setVolume, setSource, powerOn, standBy } from './services/api'
import { SelectChangeEvent } from '@mui/material/Select';
import StatusCard from './components/PowerCard';
import VolumeCard from './components/VolumeCard';
import AppBar from './components/AppBar';
import SourceCard from './components/SourceCard';

const mdTheme = createTheme();
const REFRESH_IN_MS = 1000 //1 seconds
const UPDATE_IN_MS = 3000 //3 seconds
function App() {
  const [xmcReadOnly, setXmcStatus] = useState<XmcReadOnly>({ mode: "", audioBits: "", audioBitstream: "", videoFormat: "" });
  const [xmcWrite, setXmcWrite] = useState<XmcWrite>({ power: Power.Off, source: Source.HDMI1, volume: -100 });

  const getInfo = () => getReadOnly().then(setXmcStatus)
  const getParameters = () => getWrite().then(setXmcWrite)

  const holdRefresh = useRef<undefined | ReturnType<typeof setTimeout>>(undefined)

  const getParametersLater = useCallback(() => {
    if (holdRefresh.current !== undefined) {
      clearTimeout(holdRefresh.current)
    }
    holdRefresh.current = setTimeout(() => {
      getParameters()
    }, UPDATE_IN_MS)
  }, [])

  useEffect(() => {
    setInterval(() => {
      getInfo()
    }, REFRESH_IN_MS)
  }, [])

  useEffect(() => {
    setInterval(() => {
      getParametersLater()
    }, UPDATE_IN_MS * 2)
  }, [getParametersLater])

  const onPowerToggle = () => {
    if (xmcWrite.power === Power.On) {
      setXmcWrite((prev: XmcWrite) => ({ ...prev, power: Power.Off }))
      standBy()

    }
    else if (xmcWrite.power === Power.Off) {
      setXmcWrite((prev: XmcWrite) => ({ ...prev, power: Power.On }))
      powerOn()
    }
    getParametersLater()
  }

  const onVolumeChange = (volume: number) => {
    setXmcWrite((prev: XmcWrite) => ({ ...prev, volume }))
    setVolume(volume)
    getParametersLater()
  }


  const onInputChange = (e: SelectChangeEvent) => {
    const source = e.target.value as Source
    setXmcWrite((prev: XmcWrite) => ({ ...prev, source }))
    setSource(source)
    getParametersLater()
  }

  const onVolumeUp = () => {
    volumeUp()
    setXmcWrite((prev: XmcWrite) => ({ ...prev, volume: prev.volume + 1 }))
    getParametersLater()
  }

  const onVolumeDown = () => {
    volumeDown()
    setXmcWrite((prev: XmcWrite) => ({ ...prev, volume: prev.volume - 1 }))
    getParametersLater()
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
              <Grid item xs={12} md={12} lg={12}>
                <StatusCard
                  onPowerToggle={onPowerToggle}
                  power={xmcWrite.power === Power.On}
                  onInputChange={onInputChange}
                  source={xmcWrite.source}
                />
              </Grid>
              <Grid item xs={12} md={6} lg={6}>
                <SourceCard
                  audioInfo={xmcReadOnly.audioBits}
                  audioMode={xmcReadOnly.mode}
                  audioBitstream={xmcReadOnly.audioBitstream}
                  videoFormat={xmcReadOnly.videoFormat}
                />
              </Grid>
              <Grid item xs={12} md={6} lg={6}>
                <VolumeCard
                  volumeDown={onVolumeDown}
                  volumeUp={onVolumeUp}
                  onVolumeChange={onVolumeChange}
                  volume={xmcWrite.volume}
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
