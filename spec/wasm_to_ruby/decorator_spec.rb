# frozen_string_literal: true

require 'spec_helper'
require './lib/wasm_to_ruby/decorator'

describe WasmToRuby::Decorator do
  it 'is a class' do
    expect(described_class).to be_a(Class)
  end

  context 'hello world example' do
    it 'decorates the hello world example' do
      wat = File.read('examples/test.wat')
      actual = described_class.decorate(wat)

      expected = 
       [{type:"module", value:"module"},
       [{type:"export", value:"export \"__heap_base\""},
        [{type:"global", value:"global 2"}]],
       [{type:"export", value:"export \"__data_end\""},
        [{type:"global", value:"global 1"}]],
       [{type:"export", value:"export \"_\""},
        [{type:"function", value:"func 2"}]],
       [{type:"export", value:"export \"hello\""},
        [{type:"function", value:"func 1"}]],
       [{type:"export", value:"export \"memory\""},
        [{type:"memory", value:"memory 0"}]],
       [{type:"global", value:"global  i32"},
        [{type:"statement", value:"i32.const 1048576"}],
        [{type:"comment", value:";2;"}]],
       [{type:"global", value:"global  i32"},
        [{type:"statement", value:"i32.const 1048576"}],
        [{type:"comment", value:";1;"}]],
       [{type:"global", value:"global"},
        [{type:"statement", value:"i32.const 1048576"}],
        [{type:"statement", value:"mut i32"}],
        [{type:"comment", value:";0;"}]],
       [{type:"memory", value:"memory  16"},
        [{type:"comment", value:";0;"}]],
       [{type:"function", value:"func"},
        [{type:"type", value:"type 2"}],
        [{type:"comment", value:";2;"}]],
       [{type:"import", value:"import \"v\" \"g\""},
        [{type:"function", value:"func"},
         [{type:"type", value:"type 0"}],
         [{type:"comment", value:";0;"}]]],
       [{type:"type", value:"type"},
        [{type:"function", value:"func"}],
        [{type:"comment", value:";2;"}]],
       [{type:"type", value:"type"},
        [{type:"function", value:"func"},
         [{type:"result", value:"result i64"}],
         [{type:"param", value:"param i64"}]],
        [{type:"comment", value:";1;"}]],
       [{type:"type", value:"type"},
        [{type:"function", value:"func"},
         [{type:"result", value:"result i64"}],
         [{type:"param", value:"param i64 i64"}]],
        [{type:"comment", value:";0;"}]]]



      expect(actual).to eq(expected)
    end
  end
end
