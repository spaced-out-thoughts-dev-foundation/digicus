# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRToRust::Generator do
  context 'when generating rust for the simple hello world contract' do
    let(:file_path) { 'spec/test_dtr_files/hello_world.dtr' }
    let(:generator) { described_class.new(file_path) }

    let(:expected_content) do
      <<~RUST
        #![no_std]
        use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

        #[contract]
        pub struct HelloContract;

        #[contractimpl]
        impl HelloContract {
            pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
                vec![&env, symbol_short!("Hello"), to]
            }
        }
      RUST
    end

    it 'generates valid Rust' do
      clean_up_files

      Dir.mkdir('src') unless File.exist?('src')
      File.write('src/lib.rs', generator.generate)
      write_cargo_toml

      stdout, stderr, status = compile_and_run_rust_code

      clean_up_files

      expect(status.success?).to be(true), "Rust code failed to compile: #{stderr}"
      expect(generator.generate).to eq(expected_content)
    end

    def clean_up_files
      File.delete('src/lib.rs') if File.exist?('src/lib.rs')
      File.delete('Cargo.toml') if File.exist?('Cargo.toml')
      File.delete('Cargo.lock') if File.exist?('Cargo.lock')
    end

    def write_cargo_toml
      File.write('Cargo.toml', <<~TOML
      [package]
      name = "soroban-hello-world-contract"
      version = "0.0.0"
      edition = "2021"

      [lib]
      crate-type = ["cdylib"]
      doctest = false

      [dependencies]
      soroban-sdk = { version = "20.3.1" }
      TOML
      )
    end

    # Helper method to run rustc compiler
    def compile_and_run_rust_code
      stdout, stderr, status = Open3.capture3("cargo check")
      [stdout, stderr, status]
    end
  end
end
