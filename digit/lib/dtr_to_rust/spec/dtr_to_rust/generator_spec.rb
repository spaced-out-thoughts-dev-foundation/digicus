# frozen_string_literal: true

require 'spec_helper'

RSpec.describe DTRToRust::Generator do
  let(:dtr_contract) { DTRCore::Contract.from_dtr_raw(File.read('spec/fixtures/contract.dtr')) }
  let(:generator) { described_class.new(dtr_contract) }

  describe '#generate_from_string' do
    it 'generates Rust code from a DTR contract' do
      minimal_dtr_code = <<~DTR
        [Contract]: MyContract

        [State]:
          * [COUNTER]
            * Type: String
            * Initial Value: "COUNTER"

      DTR

      expected = <<~RUST
        #![no_std]
        use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

        #[contract]
        pub struct MyContract;

        const COUNTER: Symbol = symbol_short!("COUNTER");

        #[contractimpl]
        impl MyContract {}
      RUST

      expect(described_class.generate_from_string(minimal_dtr_code)).to eq(expected)
    end
  end

  describe '#generate' do
    it 'generates Rust code from a DTR contract' do
      minimal_dtr_code = <<~DTR
        [Contract]: MyContract

        [State]:
          * [COUNTER]
            * Type: String
            * Initial Value: "COUNTER"

      DTR
      minimal_dtr_file_path = 'spec/temp.dtr'

      FileUtils.rm_f(minimal_dtr_file_path)
      File.write(minimal_dtr_file_path, minimal_dtr_code)

      expected = <<~RUST
        #![no_std]
        use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

        #[contract]
        pub struct MyContract;

        const COUNTER: Symbol = symbol_short!("COUNTER");

        #[contractimpl]
        impl MyContract {}
      RUST

      expect(described_class.generate_from_file(minimal_dtr_file_path)).to eq(expected)
    end
  end
end
