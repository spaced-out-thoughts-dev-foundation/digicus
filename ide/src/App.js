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

const App = () => {
  const [contract, setContract] = useState({ contract: '', originalText: ``, generatedText: `` });
  const [file, setFile] = useState(null);
  const [supportedInstructions, setSupportedInstructions] = useState([]);
  const [showCodeContainer, setShowCodeContainer] = useState(true);
  const [showUserDefinedTypes, setShowUserDefinedTypes] = useState(false);

  useEffect(() => {
    fetch('https://block-render-engine.vercel.app/api/supported_types_and_instructions')
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
    if (contract?.contract?.contract_functions == null) {
      return;
    }
    contract.contract.contract_functions = contract.contract?.contract_functions.map((functionData) => {
      let jsonifiedFunctionData = JSON.parse(functionData);

      if (jsonifiedFunctionData['name'] === oldTitle) {
        jsonifiedFunctionData['name'] = newTitle;
      }
      return JSON.stringify(jsonifiedFunctionData);
    });

    fetch('https://block-render-engine.vercel.app/api/generate_from_dtr',
      {
        headers: {
          'Accept': 'text/text',
          'Content-Type': 'application/json'
        },
        method: "POST",
        body: JSON.stringify({
          contract_name: contract.contract.contract_name,
          contract_state: contract.contract.contract_state,
          contract_functions: contract.contract.contract_functions,
          contract_user_defined_types: contract.contract.contract_user_defined_types
        })
      })
      .then(response => {
        return response.json()
      })
      .then(json => setContract({ contract: contract.contract, originalText: contract.originalText, generatedText: json.rust_code }))
      .catch(error => console.error(error));
  }

  const onUpdateContractName = (newTitle, _) => {
    if (contract?.contract?.contract_name == null) {
      return;
    }

    contract.contract.contract_name = newTitle;

    fetch('https://block-render-engine.vercel.app/api/generate_from_dtr',
      {
        headers: {
          'Accept': 'text/text',
          'Content-Type': 'application/json'
        },
        method: "POST",
        body: JSON.stringify({
          contract_name: contract.contract.contract_name,
          contract_state: contract.contract.contract_state,
          contract_functions: contract.contract.contract_functions,
          contract_user_defined_types: contract.contract.contract_user_defined_types
        })
      })
      .then(response => {
        return response.json()
      })
      .then(json => setContract({ contract: contract.contract, originalText: contract.originalText, generatedText: json.rust_code }))
      .catch(error => console.error(error));
  }

  const handleShowCodeContainer = () => {
    setShowCodeContainer(!showCodeContainer);
    setShowUserDefinedTypes(!showUserDefinedTypes);
  }

  const handleShowUserDefinedTypes = () => {
    setShowCodeContainer(!showCodeContainer);
    setShowUserDefinedTypes(!showUserDefinedTypes);
  }

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
        fetch('https://block-render-engine.vercel.app/api/compile_from_dtr',
          {
            headers: {
              'Accept': 'text/text',
              'Content-Type': 'application/json'
            },
            method: "POST",
            body: JSON.stringify({ format: determineFileFormat(file), content: reader.result })
          })
          .then(response => {
            return response.json()
          })
          .then(json => setContract({ contract: json, originalText: reader.result, generatedText: json.generated_code }))
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
                <FileUpload style={{ flex: 10 }} handleFileChange={handleFileChange} handleUpload={handleUpload} />
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
                    Generated Source Code
                  </label>
                </div>
              </div>
            </div>
            <ContractContainer
              functions={contract?.contract?.contract_functions}
              filename={file?.name}
              originalText={contract?.originalText}
              generatedText={contract?.generatedText}
              supportedInstructions={supportedInstructions}
              supportedInstructionToColor={supportedInstructionToColor}
              contractName={contract?.contract?.contract_name}
              showCodeContainer={showCodeContainer}
              showUserDefinedTypes={showUserDefinedTypes}
              userDefinedTypes={contract?.contract?.contract_user_defined_types}
              onUpdateFunctionName={onUpdateFunctionName}
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