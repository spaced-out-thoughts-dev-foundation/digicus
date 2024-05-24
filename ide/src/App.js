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
import React, { useEffect, useState } from 'react';
import { Box, Button, List, ListItem } from '@mui/material';
import { index } from 'd3';


const App = () => {
    const [file, setFile] = useState(null);
    const [supportedInstructions, setSupportedInstructions] = useState([]);

    const supportedInstructionToColor = (supported_instruction) => {
      console.log(supported_instruction)
      if (supported_instruction == null || supported_instruction.category == null) {
        return 'white';
      }
      if (supported_instruction.category === "basic") {
        return 'orange';
      } else if (supported_instruction.category === "state") {
        return 'red';
      } else if (supported_instruction.category === "untyped") {
        return 'silver';
      } else if (supported_instruction.category === "numeric") {
        return 'maroon';
      } else if (supported_instruction.category === "string") {
        return 'yellow';
      } else if (supported_instruction.category === "environment") {
        return 'turquoise';
      } else {
        return 'white';
      }
    };

    useEffect(() => {
      fetch('https://block-render-engine.vercel.app/api/supported_types_and_instructions')
        .then(response => {
          return response.json()
        })
        .then(json => setSupportedInstructions(json.supported_instructions))
        .catch(error => console.error(error));
    }, []);

    const handleDeploy = () => {};

    const determineFileFormat = (file) => {
      if (file == null) {
        return "";
      }

      if (file.name.endsWith(".rs")) {
        return "rust";
      }

      if (file.name.endsWith(".dtr")) {
        return "dtr";
      }

      return "unknown";
    };

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
            body: JSON.stringify({format: determineFileFormat(file), content: reader.result})
          })
          .then(response => {
            return response.json()
          })
          .then(json => setContract({ contract: json, originalText: reader.result}))
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

    const [contract, setContract] = useState({contract: '', originalText: ``});

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
                {supportedInstructions.map((supported_instruction_data, index) => (
                  <ListItem key={index} button style={{
                    backgroundColor: supportedInstructionToColor(supported_instruction_data),
                    border: '1px solid black',
                    margin: '1px',
                  }}>
                    <Button
                      style={{
                        color: 'white',
                        fontSize: '0.75em',
                        textShadow: '1px 1px 1px gray'
                      }}
                    >{<strong style={{color: 'black', fontSize: '1.25em', marginRight: '0.5em',  textShadow: '1px 1px 1px gray'}}>{supported_instruction_data.name}</strong>}{'(' + supported_instruction_data.category + ')'}</Button>
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
                name={contract?.contract?.contract_name}
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
                functions={contract?.contract?.contract_functions}
                filename={file?.name}
                originalText={contract?.originalText}
                supportedInstructions={supportedInstructions}
                supportedInstructionToColor={supportedInstructionToColor}
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