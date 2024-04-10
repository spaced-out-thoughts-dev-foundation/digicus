# frozen_string_literal: true

require './spec/spec_helper'
require './lib/parser'

RSpec.describe DTRCore::Parser do
  context 'when file does not exist' do
    it 'raises an error' do
      expect { DTRCore::Parser.parse('DTR') }.to raise_error(DTRCore::Error::FileNotFound)
    end
  end

  context 'when invalid DTR' do
    context 'when contract name section missing' do
      it 'raises an error' do
        expect do
          DTRCore::Parser.parse('./spec/test_dtr_files/missing_contract_name_section.dtr')
        end.to raise_error(DTRCore::Error::MissingContractNameSection)
      end
    end

    context 'when state section is empty' do
      it 'raises an error' do
        expect do
          DTRCore::Parser.parse('./spec/test_dtr_files/state_section_no_definitions_empty_error.dtr')
        end.to raise_error(DTRCore::Error::EmptyStateSection)
      end
    end

    context 'when state section includes a definition with missing type' do
      it 'raises an error' do
        expect do
          DTRCore::Parser.parse('./spec/test_dtr_files/state_section_missing_type_name.dtr')
        end.to raise_error(DTRCore::Error::MissingTypeName)
      end
    end

    context 'when state section includes a definition with missing initial value' do
      it 'raises an error' do
        expect do
          DTRCore::Parser.parse('./spec/test_dtr_files/state_section_missing_initial_value.dtr')
        end.to raise_error(DTRCore::Error::MissingInitialValue)
      end
    end

    context 'when state section includes an unrecognized type' do
      it 'parses the contract name section but the state section is nil' do
        expect do
          DTRCore::Parser.parse('./spec/test_dtr_files/state_section_invalid_type_name.dtr')
        end.to raise_error(DTRCore::Error::InvalidTypeName)
      end
    end
  end

  context 'when valid DTR' do
    context 'when only contract name section is present' do
      it 'parses the contract name section' do
        parser = DTRCore::Parser.parse('./spec/test_dtr_files/contract_name_section_only.dtr')

        expect(parser[:contract_name]).to eq('CONTRACT_NAME')

        # empty contract so these optional sections are nil
        expect(parser[:state]).to be_nil
        expect(parser[:functions]).to be_nil
      end

      it 'parses the name of the contract even when it is weird' do
        parser = DTRCore::Parser.parse('./spec/test_dtr_files/contract_name_section_only_weird_name.dtr')

        expect(parser[:contract_name]).to eq('CONTRACT_NAME is foo 123')

        # empty contract so these optional sections are nil
        expect(parser[:state]).to be_nil
        expect(parser[:functions]).to be_nil
      end
    end

    context 'when contract name and state sections are present' do
      it 'parses the contract name and state sections' do
        parser = DTRCore::Parser.parse('./spec/test_dtr_files/state_section_simple_no_functions.dtr')

        expect(parser[:contract_name]).to eq('CONTRACT_NAME')

        expect(parser[:state]).to match_array([
                                                { name: 'STATE_DEFINITION_1', type: 'I32', initial_value: 22 },
                                                { name: 'STATE_DEFINITION_2', type: 'Symbol',
                                                  initial_value: 'Hello World' },
                                                { name: 'STATE_DEFINITION_3', type: 'I256', initial_value: -1234 }
                                              ])

        # empty contract so these optional sections are nil
        expect(parser[:functions]).to be_nil
      end
    end
  end
end
