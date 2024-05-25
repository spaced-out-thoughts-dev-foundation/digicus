import React from 'react'
import Box from '@mui/material/Box';
import ReactFlow, { Controls, MarkerType } from 'reactflow';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { dracula } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { rust } from 'react-syntax-highlighter/dist/esm/languages/prism';
import InstructionNode from './InstructionNode';

const nodeTypes = {
  instructionNode: InstructionNode,
};

function all_nodes(function_data, supportedInstructions, supportedInstructionToColor, function_number) {
  let function_json_data = JSON.parse(function_data);
  let all_function_nodes = function_json_data.instructions
    .filter((instruction) => !!supportedInstructions.filter(x => x.name.toUpperCase() === instruction.instruction.toUpperCase()))
    .map((instruction, index) => {
      const instructionData = supportedInstructions.filter(x => x.name.toUpperCase() === instruction.instruction.toUpperCase())[0];
      index += 1;
      return { id: `${index}|${function_number}`, style: { 
        backgroundColor:  supportedInstructionToColor(instructionData),
        fontSize: '0.5em',
      }, data: 
      { instruction: instruction, label: `${instructionData.name.toUpperCase()} ${instruction.inputs ? `(${instruction.inputs.join(',')})` : ''}` }, 
      type: 'instructionNode', 
      position: { x: 200 * (function_number), y: 150 * (index + 1) },
      // parentId: `f-${function_number}`,
      }
    }) 

    // all_function_nodes.push(
    //   { id: `f-${function_number}`, style: { 
    //     backgroundColor: 'white',
    //     textShadow: '1px 1px 1px gray',
    //     fontSize: '0.5em',
    //   }, 
    //   data: { label: `FUNCTION: [${function_json_data.name}]` }, 
    //   position: { x: 200 * (function_number), y: 0},
    //   style: {
    //     backgroundColor: 'pink',
    //     height: 150 * (function_json_data.instructions.length + 2),
    //     width: 200,
    //   },
    //   type: 'group',
    //   className: 'light',
    //   extent: 'parent',

    // })
      
    
    return all_function_nodes;
}

function all_edges(function_data, function_number) {
  let function_json_data = JSON.parse(function_data);

  let all_function_edges = function_json_data.instructions.slice(1).map((instruction, index) => {
    return { 
      id: `e-${index + 1}|${function_number}`, 
      source: `${index + 1}|${function_number}`, 
      target: `${index + 2}|${function_number}`, 
      markerEnd: {
        type: MarkerType.ArrowClosed,
        width: 20,
        height: 20
      }}
  })

  return all_function_edges;
};

function all_function_nodes(functions_data, supportedInstructions, supportedInstructionToColor) {
  return functions_data.map((function_data, index) => {
    return all_nodes(function_data, supportedInstructions, supportedInstructionToColor, index);
  }).flatMap(x => x);
};

function all_function_edges(functions_data) {
  return functions_data.map((function_data, index) => {
    return all_edges(function_data, index);
  }).flatMap(x => x);
};

// TODO: this only works for one function right now
function ContractContainer({functions, supportedInstructions, supportedInstructionToColor, originalText, filename}) {
    console.log(functions);
    return (
      <div style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          flexDirection: 'row',
          flex: '15',
          backgroundColor: 'rgb(39 207 230)',
      }}>
        <Box
            style={{
              backgroundColor: 'white',
              color: 'black',
              borderRadius: '10px',
              flex: 10,
              height: '95%',
            }}
            height={'100%'}
            width={'80%'}
            my={4}
            display="flex"
            alignItems="center"
            gap={4}
            p={2}
            sx={{ border: '2px solid grey' }}
        >
          {
            functions ?
                  <ReactFlow 
                    nodes={all_function_nodes(functions, supportedInstructions, supportedInstructionToColor)} 
                    edges={all_function_edges(functions)}
                    fitView={{padding: 10}}
                    panOnDrag={false}
                    zoomOnScroll={false}
                    zoomOnPinch={false}
                    zoomOnDoubleClick={false}
                    nodeTypes={nodeTypes}
                    >
                </ReactFlow>
               : ''
          } 
        </Box>

        <Box
            style={{
              backgroundColor: 'black',
              color: 'black',
              borderRadius: '10px',
              flex: 3,
              margin: '20px',
              height: '95%',
              color: "white",
              flexDirection: 'column',
            }}
            height={'100%'}
            width={'80%'}
            my={4}
            display="flex"
            alignItems="center"
            gap={4}
            p={2}
            sx={{ border: '2px solid grey' }}
        >
           <h1>{filename && originalText ? filename : ''} </h1>
          {
            filename && originalText ? 
              <SyntaxHighlighter language="rust" style={dracula}>
                {originalText}
              </SyntaxHighlighter> : ''
          }
         
        </Box>
      </div>
    );
}
export default ContractContainer;