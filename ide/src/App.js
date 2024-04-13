// import BarChart from './BarChart'
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Avatar from '@mui/material/Avatar';
import { SpeedInsights } from "@vercel/speed-insights/react"
import { Analytics } from "@vercel/analytics/react"
import ContractContainer from './components/ContractContainer'

const App = () => {
    return ( 
      <div style={{
        height: '100%',
        width: '100%',
        display: 'flex',
      }}>
        <AppBar position="static" style={{
        height: '100%',
        width: '100%',
        display: 'flex',
        backgroundColor: 'black',
      }}>
          <Toolbar style={{ 
              display: 'flex', 
              backgroundColor: 'black',
              // flex: 1,
            }}>
            <div style={{flex: 1}}>
              <Avatar alt="Logo" src="DevelopmentFoundation_copy_512x575.png" />
            </div>
            <div style={{
                flex: 1, 
                display: 'flex', 
                justifyContent: 'center',
                color: 'rgb(39 207 230)'
              }}>
              <Typography style={{fontSize: '1.5em'}}>
                Digicus IDE
              </Typography>
            </div>
            <div style={{flex: 1}}></div>
          
            </Toolbar>

          <div style={{ flex: 1, width: '100%', height: '100%'}}>
            <ContractContainer />
          </div>
          
        </AppBar>

        <SpeedInsights />
        <Analytics />
      </div>
    )
}
export default App;