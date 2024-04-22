// import BarChart from './BarChart'
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Avatar from '@mui/material/Avatar';
import { SpeedInsights } from "@vercel/speed-insights/react"
import { Analytics } from "@vercel/analytics/react"
import ContractContainer from './components/ContractContainer'
import ContractHeader from './components/ContractHeader'
import InfoHeader from './components/InfoHeader'

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
            <div style={{flex: 1, justifyContent: 'right', display: 'flex'}}>
              <InfoHeader />
            </div>
          
            </Toolbar>

          <div style={{ 
            display: 'flex', 
            flex: 1, 
            width: '100%', 
            height: '100%',
            flexDirection: 'row',
            backgroundColor: 'rgb(39 207 230)',

          }}>
            <div style={{ 
            display: 'flex', 
            flex: 1, 
            width: '80%', 
            height: '90%',
            flexDirection: 'column',
            backgroundColor: 'rgb(39 207 230)',
            justifyContent: 'center',
            alignContent: 'center',
            alignItems: 'center',
            border: '5px solid black',
            padding: '1%',
            margin: '1%',
          }}>
              <h1>Menu</h1>
            </div>
            <div style={{ 
            display: 'flex', 
            flex: 5, 
            width: '100%', 
            height: '100%',
            flexDirection: 'column',
          }}>
              <ContractHeader />
              <ContractContainer />
            </div>
          </div>
          
        </AppBar>

        <SpeedInsights />
        <Analytics />
      </div>
    )
}
export default App;