import React from 'react'
import Box from '@mui/material/Box';
import ReactFlow, { Controls, MarkerType } from 'reactflow';

// TODO: this only works for one function right now
function ContractContainer({functions, supportedInstructions, supportedInstructionToColor}) {
    return (
      <div style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          flexDirection: 'column',
          flex: '15',
          backgroundColor: 'rgb(39 207 230)',
      }}>
        <Box
            style={{
              backgroundColor: 'white',
              color: 'black',
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
              <ReactFlow nodes={JSON.parse(functions[0]).instructions.split(' ')
              .filter((instructionName) => !!supportedInstructions.filter(x => x.name.toUpperCase() === instructionName.toUpperCase()))
              .map((instructionName, index) => {
                const instructionData = supportedInstructions.filter(x => x.name.toUpperCase() === instructionName.toUpperCase())[0];

                return { id: `${index}`, style: { 
                  backgroundColor:  supportedInstructionToColor(instructionData),
                  textShadow: '1px 1px 1px gray',
                }, data: { label: instructionData.name.toUpperCase() }, position: { x: 100, y: 100 * index } }
                  })} edges={JSON.parse(functions[0]).instructions.split(' ').slice(1).map((instructionName, index) => {
                    return { 
                      id: `e-${index}`, 
                      source: `${index}`, 
                      target: `${index + 1}`, 
                      markerEnd: {
                        type: MarkerType.ArrowClosed,
                        width: 20,
                        height: 20
                      }}
                })}
                fitView={true}
                panOnDrag={false}
                zoomOnScroll={false}
                zoomOnPinch={false}
                zoomOnDoubleClick={false}
                >
            </ReactFlow> : ''
          } 
        </Box>
      </div>
    );
}
export default ContractContainer;