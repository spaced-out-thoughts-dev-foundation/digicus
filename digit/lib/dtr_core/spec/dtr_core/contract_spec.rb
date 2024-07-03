# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRCore::Contract do
  context 'when passing filepath' do
    let(:hello_function) do
      DTRCore::Function.new('hello', [
                              { name: 'to', type_name: 'String' },
                              { name: 'from', type_name: 'Integer' }
                            ], 'String', [
                              DTRCore::Instruction.new('add', ['"Hello"', 'to'], 'HelloToResult', 0),
                              DTRCore::Instruction.new('return', ['HelloToResult'], nil, 0)
                            ])
    end

    let(:world_function) do
      DTRCore::Function.new('world', [], 'String', [
                              DTRCore::Instruction.new('return', nil, 'ReturnValue', 0)
                            ])
    end

    let(:state_definitions) do
      [
        DTRCore::State.new('STATE_DEFINITION_1', 'Integer', 22),
        DTRCore::State.new('STATE_DEFINITION_2', 'String', '"Hello World"'),
        DTRCore::State.new('STATE_DEFINITION_3', 'BigInteger', -1234)
      ]
    end

    let(:bar_helper_function) do
      DTRCore::Function.new('bar_helper', [], 'String', [
                              DTRCore::Instruction.new('return', nil, 'ReturnValue', 0)
                            ])
    end

    let(:user_defined_types) do
      [
        DTRCore::UserDefinedType.new('State', [
                                       { name: 'count', type: 'Integer' },
                                       { name: 'last_incr', type: 'Integer' }
                                     ]),
        DTRCore::UserDefinedType.new('State_Two', [
                                       { name: 'name', type: 'String' }
                                     ])
      ]
    end

    it 'parses the contract name, state, and function sections' do
      contract = described_class.from_dtr('./spec/test_dtr_files/multi_function_with_state_and_name_contract.dtr')

      expect(contract.name).to eq('MultiFunctionContract')
      expect(contract.state).to match_array(state_definitions)
      expect(contract.interface).to contain_exactly(hello_function, world_function)
    end

    it 'parses the contract, then generates the contract, then parses the generated contract' do
      contract = described_class.from_dtr('./spec/test_dtr_files/multi_function_with_state_and_name_contract.dtr')

      expect(contract.name).to eq('MultiFunctionContract')

      expect(contract.state).to match_array(state_definitions)
      expect(contract.interface).to contain_exactly(hello_function, world_function)

      generated_content = contract.to_s

      generated_contract = described_class.from_dtr_raw(generated_content)

      expect(generated_contract).to eq(contract)
    end

    it 'parses the contract, then generates the contract, then parses the generated contract, nearly empty contract' do
      contract = described_class.from_dtr('./spec/test_dtr_files/contract_name_section_only.dtr')

      expect(contract.name).to eq('CONTRACT_NAME')
      expect(contract.state).to be_nil
      expect(contract.interface).to be_nil

      generated_content = contract.to_s

      generated_contract = described_class.from_dtr_raw(generated_content)

      expect(generated_contract).to eq(contract)
    end

    it 'parses a contract with all sections' do
      contract = described_class.from_dtr('./spec/test_dtr_files/all_sections_contract.dtr')

      expect(contract.name).to eq('AllSectionsContract')
      expect(contract.state).to match_array(state_definitions)
      expect(contract.interface).to contain_exactly(hello_function, world_function)
      expect(contract.helpers).to contain_exactly(bar_helper_function)
      expect(contract.user_defined_types).to eq(user_defined_types)

      generated_content = contract.to_s

      generated_contract = described_class.from_dtr_raw(generated_content)

      expect(generated_contract).to eq(contract)
    end

    context 'when contract has double sections' do
      it 'raises an error when two contract names are found' do
        expect do
          described_class.from_dtr('./spec/test_dtr_files/double_contract_name_section.dtr')
        end.to raise_error(/Multiple captures found for a section./)
      end
    end

    context 'when contract has four of a single section' do
      it 'raises an error when four state sections are found' do
        expect do
          described_class.from_dtr('./spec/test_dtr_files/quadruple_state_section.dtr')
        end.to raise_error(/Multiple captures found for a section./)
      end
    end
  end

  context 'when passing content directly' do
    let(:hello_function) do
      DTRCore::Function.new('hello', [
                              { name: 'to', type_name: 'String' },
                              { name: 'from', type_name: 'Integer' }
                            ], 'String', [
                              DTRCore::Instruction.new('add', ['"Hello"', 'to'], 'HelloToResult', 0),
                              DTRCore::Instruction.new('return', ['HelloToResult'], nil, 0)
                            ])
    end

    let(:world_function) do
      DTRCore::Function.new('world', [], 'String', [
                              DTRCore::Instruction.new('return', nil, 'ReturnValue', 0)
                            ])
    end

    let(:state_definitions) do
      [
        DTRCore::State.new('STATE_DEFINITION_1', 'Integer', 22),
        DTRCore::State.new('STATE_DEFINITION_2', 'String', '"Hello World"'),
        DTRCore::State.new('STATE_DEFINITION_3', 'BigInteger', -1234)
      ]
    end

    let(:simplified_state_definitions) do
      [
        DTRCore::State.new('STATE_DEFINITION_1', 'Integer', 22)

      ]
    end

    it 'parses the contract name, state, and function sections_for_multi_function_with_state_and_name_contract' do
      content = File.read('./spec/test_dtr_files/multi_function_with_state_and_name_contract.dtr')

      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq('MultiFunctionContract')
      expect(contract.state).to match_array(state_definitions)
      expect(contract.interface).to contain_exactly(hello_function, world_function)
    end

    it 'parses the contract name and state when passed with explicit newline and tab characters' do
      content = "[Contract]: Foo \n"
      content += "[State]:\n\t* [STATE_DEFINITION_1]\n\t\t* Type: Integer\n\t\t* Initial Value: 22\n:[State]"

      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq('Foo')
      expect(contract.state).to match_array(simplified_state_definitions)
    end

    it 'parses the contract name, state, and function sections_for_increment_answer_to_life_contract' do
      content = File.read('./spec/test_dtr_files/increment_answer_to_life_contract.dtr')

      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq('IncrementAnswerToLifeContract')
      expect(contract.interface).to contain_exactly(
        DTRCore::Function.new(
          'fourty_two_and_then_some',
          [{ name: 'and_then_some', type_name: 'Integer' }],
          'Integer',
          [
            DTRCore::Instruction.new('assign', ['42'], 'BINARY_EXPRESSION_LEFT', 0),
            DTRCore::Instruction.new('assign', ['and_then_some'], 'BINARY_EXPRESSION_RIGHT', 0),
            DTRCore::Instruction.new('add', %w[BINARY_EXPRESSION_LEFT BINARY_EXPRESSION_RIGHT], 'Thing_to_return', 0),
            DTRCore::Instruction.new('return', ['Thing_to_return'], nil, 0)
          ]
        )
      )
    end
  end

  context 'when inputs are strings with commas' do
    it 'parses the contract name and function sections' do
      expected_function =   DTRCore::Function.new(
        'hello',
        [{ name: 'to', type_name: 'Symbol' }],
        'Symbol',
        [
          DTRCore::Instruction.new('add', ['"Hello, world, how are you?,"', 'to'], 'Thing_to_return', 0),
          DTRCore::Instruction.new('return', ['Thing_to_return'], nil, 0)
        ]
      )
      content = File.read('./spec/test_dtr_files/hello_world_simple_with_commas_in_string.dtr')
      contract = described_class.from_dtr_raw(content)
      expect(contract.name).to eq('HelloContract')
      expect(contract.interface).to contain_exactly(
        expected_function
      )
    end
  end

  context 'when parsing log_if_answer_to_life contract' do
    let(:contract_name) { 'LogIfAnswerToLife' }
    let(:interface) do
      [
        DTRCore::Function.new(
          'fourty_two_and_then_some',
          [{ name: 'env', type_name: 'Env' },
           { name: 'possibly_the_answer_to_life', type_name: 'Integer' }],
          nil,
          [
            DTRCore::Instruction.new('evaluate', %w[equal_to possibly_the_answer_to_life ANSWER_TO_LIFE],
                                     'UNARY_ARGUMENT_0', 0),
            DTRCore::Instruction.new('evaluate', ['!', 'UNARY_ARGUMENT_0'], 'CONDITIONAL_JUMP_ASSIGNMENT', 0),
            DTRCore::Instruction.new('jump', %w[CONDITIONAL_JUMP_ASSIGNMENT 1], nil, 0),
            DTRCore::Instruction.new('evaluate', ['log_to_env', 'env', '"Yes, the answer to life is 42!"'], nil, 1)
          ]
        )
      ]
    end
    let(:state) do
      [
        DTRCore::State.new('ANSWER_TO_LIFE', 'Integer', 42)
      ]
    end
    let(:helpers) do
      [
        DTRCore::Function.new(
          'log_to_env',
          [{ name: 'env', type_name: 'Env' },
           { name: 'message', type_name: 'String' }],
          nil,
          [
            DTRCore::Instruction.new('print', %w[env message], nil, 0)
          ]
        )
      ]
    end

    it 'parses each section' do
      content = File.read('./spec/test_dtr_files/log_if_answer_to_life.dtr')
      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq(contract_name)
      expect(contract.interface).to match_array(interface)
      expect(contract.state).to match_array(state)
      expect(contract.helpers).to match_array(helpers)
    end
  end

  context 'when UDT with numbered enum' do
    it 'parses each section' do
      content = File.read('./spec/test_dtr_files/contract_name_section_with_numbered_enum.dtr')
      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq('CONTRACT_NAME')
      expect(contract.user_defined_types).to eq(
        [
          DTRCore::UserDefinedType.new('Error_ENUM', [
                                         { name: 'LimitReached', type: '1' }
                                       ])
        ]
      )
    end
  end

  context 'when UDT with enum variants' do
    it 'parses each section' do
      content = File.read('./spec/test_dtr_files/contract_name_section_with_enum_variants.dtr')
      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq('CONTRACT_NAME')
      expect(contract.user_defined_types).to eq(
        [
          DTRCore::UserDefinedType.new('DataKey_ENUM', [
                                         { name: 'Counter', type: '(Address)' },
                                         { name: 'Counter2', type: '(Address, BigInteger)' },
                                         { name: 'Counter3', type: '()' }
                                       ])
        ]
      )
    end
  end

  context 'when non-translatable section' do
    it 'parses each section' do
      content = File.read('./spec/test_dtr_files/contract_and_non_translatable_section.dtr')
      contract = described_class.from_dtr_raw(content)

      expect(contract.name).to eq('CONTRACT_NAME')
      expect(contract.non_translatables.gsub(' ', '').gsub("\n", '').gsub("\t", '')).to eq(
        'mod contract_a {
          soroban_sdk::contractimport!(
            file = "../contract_a/target/wasm32-unknown-unknown/release/soroban_cross_contract_a_contract.wasm"
          );
        }'.gsub(' ', '').gsub("\n", '').gsub("\t", '')
      )
    end
  end
end
