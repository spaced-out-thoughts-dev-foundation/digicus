# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Answer to Life' do
  context 'when translating Answer to Life contract from Rust --> DTR --> Rust' do
    it 'passes the tests' do
      dir = './stellar_soroban/unofficial_digicus_examples/answer_to_life/'

      rust_string = DTRToRust::Generator.generate_from_string(rust_to_dtr(File.read("#{dir}/src/original.rs")))
      rust_string += "\n\nmod test;\n"
      File.write("#{dir}/src/lib.rs", rust_string)

      expect(run_cargo_test_in_dir(dir)).to be true
    end
  end
end
