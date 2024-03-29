# frozen_string_literal: true

require 'spec_helper'
require './lib/parser'

RSpec.describe Digit::Parser do
  describe '#initialize' do
    it 'creates a new instance of Parser' do
      expect(Digit::Parser.new('./examples/empty.rs')).to be_an_instance_of(Digit::Parser)
    end

    context 'when given empty contract example' do
      it 'parses the file' do
        filepath = './examples/empty.rs'

        actual = Digit::Parser.parse(filepath)

        expected = [
          { type: :no_std_header },
          { type: :use_statement, value: 'soroban_sdk::{contract, contractimpl}' },
          { type: :contract_header },
          { type: :struct, name: 'EmptyContract' },
          { type: :contract_impl_header },
          { type: :contract_impl, name: 'EmptyContract', state: nil, functions: [] }
        ]

        expect(actual).to eq(expected)
      end
    end
  end
end
