// import BarChart from './BarChart'
import AppBar from '@mui/material/AppBar';
import { SpeedInsights } from "@vercel/speed-insights/react"
import { Analytics } from "@vercel/analytics/react"
import ContractContainer from './components/ContractContainer'
import ContractHeader from './components/ContractHeader'
import React, { useEffect, useState } from 'react';
import { supportedInstructionToColor } from './common/InstructionNode';
import FileUpload from './components/FileUpload';
import TopHeaderBar from './components/TopHeaderBar';
import InstructionsAndActionsSideBar from './components/InstructionsAndActionsSideBar';
import { saveAs } from 'file-saver';
import EmailForm from './components/EmailForm';
import { localContractFetch } from './common/LocalContractFetcher';

const App = () => {
  const [contract, setContract] = useState({ contract: '', originalText: ``, generatedText: `` });
  const [file, setFile] = useState('hello_world.rs');
  const [supportedInstructions, setSupportedInstructions] = useState([]);
  const [showCodeContainer, setShowCodeContainer] = useState(true);
  const [showUserDefinedTypes, setShowUserDefinedTypes] = useState(false);

  console.log("BASE_URL: ", process.env.BASE_URL);

  useEffect(() => {
    fetch(`${process.env.BASE_URL}/api/supported_types_and_instructions`)
      .then(response => {
        return response.json()
      })
      .then(json => setSupportedInstructions(json.supported_instructions))
      .catch(error => console.error(error));
  }, []);

  const handleDeploy = () => {
    var blob = new Blob([contract.generatedText], { type: "text/plain;charset=utf-8" });
    saveAs(blob, `${contract?.contract?.contract_name}_generated_contract.rs`);
  };

  const onUpdateFunctionName = (newTitle, oldTitle) => {
    // if (contract?.contract?.contract_functions == null) {
    //   return;
    // }
    // contract.contract.contract_functions = contract.contract?.contract_functions.map((functionData) => {
    //   let jsonifiedFunctionData = JSON.parse(functionData);

    //   if (jsonifiedFunctionData['name'] === oldTitle) {
    //     jsonifiedFunctionData['name'] = newTitle;
    //   }
    //   return JSON.stringify(jsonifiedFunctionData);
    // });

    // fetch('https://block-render-engine.vercel.app/api/generate_from_dtr',
    //   {
    //     headers: {
    //       'Accept': 'text/text',
    //       'Content-Type': 'application/json'
    //     },
    //     method: "POST",
    //     body: JSON.stringify({
    //       contract_name: contract.contract.contract_name,
    //       contract_state: contract.contract.contract_state,
    //       contract_functions: contract.contract.contract_functions,
    //       contract_user_defined_types: contract.contract.contract_user_defined_types
    //     })
    //   })
    //   .then(response => {
    //     return response.json()
    //   })
    //   .then(json => setContract({ contract: contract.contract, originalText: contract.originalText, generatedText: json.rust_code }))
    //   .catch(error => console.error(error));
  }

  const onUpdateContractName = (newTitle, _) => {
    // if (contract?.contract?.contract_name == null) {
    //   return;
    // }

    // contract.contract.contract_name = newTitle;

    // fetch('https://block-render-engine.vercel.app/api/generate_from_dtr',
    //   {
    //     headers: {
    //       'Accept': 'text/text',
    //       'Content-Type': 'application/json'
    //     },
    //     method: "POST",
    //     body: JSON.stringify({
    //       contract_name: contract.contract.contract_name,
    //       contract_state: contract.contract.contract_state,
    //       contract_functions: contract.contract.contract_functions,
    //       contract_user_defined_types: contract.contract.contract_user_defined_types
    //     })
    //   })
    //   .then(response => {
    //     return response.json()
    //   })
    //   .then(json => setContract({ contract: contract.contract, originalText: contract.originalText, generatedText: json.rust_code }))
    //   .catch(error => console.error(error));
  }

  const onUpdateInputName = (newTitle, oldTitle, instruction, input_index, function_number, instruction_index) => {
    // let jsonifiedFunctionData = JSON.parse(contract.contract.contract_functions[function_number]);
    // let jsonifiedInstructionData = JSON.parse(jsonifiedFunctionData.instructions[instruction_index - 1]);

    // jsonifiedInstructionData.inputs[input_index] = newTitle;
    // jsonifiedFunctionData.instructions[instruction_index - 1] = JSON.stringify(jsonifiedInstructionData);

    // contract.contract.contract_functions[function_number] = JSON.stringify(jsonifiedFunctionData);

    // fetch('https://block-render-engine.vercel.app/api/generate_from_dtr',
    //   {
    //     headers: {
    //       'Accept': 'text/text',
    //       'Content-Type': 'application/json'
    //     },
    //     method: "POST",
    //     body: JSON.stringify({
    //       contract_name: contract.contract.contract_name,
    //       contract_state: contract.contract.contract_state,
    //       contract_functions: contract.contract.contract_functions,
    //       contract_user_defined_types: contract.contract.contract_user_defined_types
    //     })
    //   })
    //   .then(response => {
    //     return response.json()
    //   })
    //   .then(json => setContract({ contract: contract.contract, originalText: contract.originalText, generatedText: json.rust_code }))
    //   .catch(error => console.error(error));
  };

  const handleShowCodeContainer = () => {
    setShowCodeContainer(!showCodeContainer);
    setShowUserDefinedTypes(!showUserDefinedTypes);
  }

  const handleShowUserDefinedTypes = () => {
    setShowCodeContainer(!showCodeContainer);
    setShowUserDefinedTypes(!showUserDefinedTypes);
  }

  const handleUpload = (contract) => {
    let contractText = localContractFetch(contract);
    fetch(`${process.env.BASE_URL}/api/compile`,
      {
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
        method: "POST",
        body: JSON.stringify({ name: "soroban_rust_frontend", type: "frontend", content: contractText }),
      })
      .then(response => {
        return response.json()
      })
      .then(response => {
        let dtr_code = response.output;

        fetch(`${process.env.BASE_URL}/api/compile`,
          {
            headers: {
              'Accept': 'application/json',
              'Content-Type': 'application/json'
            },
            method: "POST",
            body: JSON.stringify({ name: "digicus_web_backend", type: "backend", content: dtr_code }),
          })
          .then(response => {
            return response.json()
          })
          .then(json => {
            console.log("JSON: ", json);
            setContract({ contract: JSON.parse(json.output), originalText: contractText, generatedText: dtr_code })
          })
          .catch(error => console.error(error));
      })
      .catch(error => console.error(error));
  };

  console.log("Contract Name:", contract?.contract?.contract_name)

  return (
    <div className='top-level-div-container'>
      <AppBar position="static" className='top-app-bar'>
        <TopHeaderBar />
        <div className='top-level-second-level-container'>
          <InstructionsAndActionsSideBar handleDeploy={handleDeploy} supportedInstructions={supportedInstructions} />
          <div className='top-level-third-level-container'>
            <div className='top-level-third-level-container-secondary-header-bar'>
              <ContractHeader name={contract?.contract?.contract_name} onUpdateContractName={onUpdateContractName} />
              <div style={{ display: 'flex', flexDirection: 'column' }}>
                <FileUpload style={{ flex: 10 }} handleUpload={handleUpload} />
                <div style={{ border: '1px solid black', borderRadius: '10px', margin: '10px' }}>
                  <label>
                    <input
                      type="checkbox"
                      checked={showCodeContainer}
                      onChange={handleShowCodeContainer}
                    />
                    Original Source Code
                  </label>
                  <br></br>
                  <label>
                    <input
                      type="checkbox"
                      checked={showUserDefinedTypes}
                      onChange={handleShowUserDefinedTypes}
                    />
                    Generated Intermediate Code
                  </label>
                </div>
              </div>
            </div>
            {/* <div style={{
              flex: 1,
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
            }}>
              <div style={{ backgroundColor: 'blue', padding: '10px', fontSize: '1.5em', borderRadius: '10px' }}>
                <h3>Thank you for your interest in Digicus! <br />We're undergoing a facelift in preparation for our initial launch. <br />In the meantime...</h3>
                <EmailForm />
              </div>
            </div> */}

            <ContractContainer
              functions={contract?.contract?.contract_interface}
              filename={file}
              originalText={contract?.originalText}
              generatedText={contract?.generatedText}
              supportedInstructions={supportedInstructions}
              supportedInstructionToColor={supportedInstructionToColor}
              contractName={contract?.contract?.contract_name}
              showCodeContainer={showCodeContainer}
              showUserDefinedTypes={showUserDefinedTypes}
              userDefinedTypes={contract?.contract?.contract_user_defined_types}
              onUpdateFunctionName={onUpdateFunctionName}
              onUpdateInputName={onUpdateInputName}
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