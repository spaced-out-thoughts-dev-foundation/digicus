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
import { Box, Button, List, ListItem } from '@mui/material';
import { index } from 'd3';


const App = () => {
    const [file, setFile] = useState(null);

    const handleDeploy = () => {};

    const options = [
      'Instruction Operation 1',
      'Instruction Operation 2',
      'Instruction Operation 3',
      'Instruction Operation 4',
      'Instruction Operation 5',
      'Instruction Operation 6',
      'Instruction Operation 7',
      'Instruction Operation 8',
      'Instruction Operation 9',
      'Instruction Operation 10',
      'Instruction Operation 11',
      'Instruction Operation 12',
      'Instruction Operation 13',
      'Instruction Operation 14',
      'Instruction Operation 15',
      'Instruction Operation 16',
      'Instruction Operation 17',
      'Instruction Operation 18',
      'Instruction Operation 19',
      'Instruction Operation 20',
      'Instruction Operation 21',
      'Instruction Operation 22',
      'Instruction Operation 23',
      'Instruction Operation 24',
      'Instruction Operation 25',
      'Instruction Operation 26',
      'Instruction Operation 27',
      'Instruction Operation 28',
      'Instruction Operation 29',
      'Instruction Operation 30',
    ];
    

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
            height: '90%',
            flexDirection: 'row',
            backgroundColor: 'rgb(39 207 230)',

          }}>
          <div style={{
            display: 'flex',
            flex: 1,
            flexDirection: 'column',
            height: '100%',
          }}>
            <div style={{ 
              flex: 2, 
              justifyContent: 'center',
              alignContent: 'center',
              alignItems: 'center',
              border: '1px solid white',
              padding: '1%',
              margin: '1%',
              overflowY: 'auto',
              height: '100%',
              align: 'center',
              textAlign: 'center',
            }}>
              <h2>Instructions Menu</h2>
            </div>
              <div style={{ 
              flex: 19, 
              justifyContent: 'center',
              alignContent: 'center',
              alignItems: 'center',
              border: '5px solid black',
              padding: '1%',
              margin: '1%',
              overflowY: 'auto',
              height: '100%',
            }}>
              <List>
                {options.map((option, index) => (
                  <ListItem key={index} button style={{
                    backgroundColor: index  % 2 === 0 ? 'white' : 'gray',
                    border: '1px solid black',
                    margin: '1px',
                  }}>
                    <Button
                      style={{
                        color: 'black',
                      }}
                     >{option}</Button>
                  </ListItem>
                ))}
              </List>

              </div>
            <div style={{
              flex: 1,
              display: 'flex', 
              padding: '1%',
              margin: '5%',
              }}>
              <button style={{width: '100%'}} onClick={handleDeploy}>Deploy</button>
            </div>

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
                <input style={{fontSize: '1.25em', backgroundColor: 'white', margin: '2%', color: 'black'}} type="file" onChange={handleFileChange} />
                <button style={{flex: 1, fontSize: '1.25em'}}onClick={handleUpload}>Upload</button>
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