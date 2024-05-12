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
import React, { useState } from 'react';

const App = () => {
    const [file, setFile] = useState(null);

    const handleUpload = () => {
      const reader = new FileReader();

      reader.addEventListener(
        "load",
        () => {
          // this will then display a text file
          console.log(reader.result);
          fetch('https://block-render-engine.vercel.app/api/compile_from_dtr',
          {
            headers: {
              'Accept': 'text/text',
              'Content-Type': 'application/json'
            },
            method: "POST",
            body: JSON.stringify({format: "dtr", content: reader.result})
          })
          .then(response => {
            return response.json()
          })
          .then(json => setContract(json))
          .catch(error => console.error(error));
        },
        false,
      );
    
      if (file) {
        reader.readAsText(file);
      };
    };

    const handleFileChange = (event) => {
      const selectedFile = event.target.files[0];
      setFile(selectedFile);
    };

    const [contract, setContract] = useState(null);

    // useEffect(() => {
    //   []);

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
            flexDirection: 'column',
          }}>
            
            <div style={{ 
            display: 'flex', 
            flex: 2, 
            flexDirection: 'row',
            justifyContent: 'center',
            alignContent: 'center',
            alignItems: 'center',
          }}>
              <ContractHeader
                name={contract?.contract_name}
              />
              <div style={{ 
                backgroundColor: 'gray',
                display: 'flex', 
                flex: 1, 
                justifyContent: 'center',
                alignContent: 'center',
                alignItems: 'center',
                // width: '100%', 
                height: '50%',
                margin: '10px',
                padding: '10px',
                boxShadow: '5px 5px 5px black',
                // flexDirection: 'row',
              }}>
                <input type="file" onChange={handleFileChange} />
                <button style={{flex: 1}}onClick={handleUpload}>Upload</button>
              </div>
            </div>
             
              
              <ContractContainer 
                functions={contract?.contract_functions}
              />
            </div>
          </div>
          
        </AppBar>

        <SpeedInsights />
        <Analytics />
      </div>
    )
}
export default App;