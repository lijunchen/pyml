import { useState, useEffect } from 'react';
import Editor from '@monaco-editor/react';
import { execute, compile_to_core } from 'wasm-app';

function App() {
  const [code, setCode] = useState("let a = print_int(123) in a");
  const [result, setResult] = useState("");
  const [core, setCore] = useState("");

  useEffect(() => {
    try {
      const result = execute(code);
      console.log(result);
      setResult(result);

      const core = compile_to_core(code)
      console.log(core);
      setCore(core);
    } catch (error) {
      console.error(error);
    }
  }, [code]);

  return (
    <div className="h-screen flex">
      <div className="w-1/2 border-r border-gray-300 flex flex-col">
        <div className="flex-1">
          <Editor
            height="100%"
            value={code}
            onChange={(value) => setCode(value || "")}
            options={{
              fontSize: 14,
              minimap: { enabled: false },
              automaticLayout: true,
            }}
          />
        </div>
      </div>

      <div className="w-1/2 flex flex-col h-full">
        <div className="flex-grow h-[80%] overflow-auto p-4">
          <h2 className="text-xl font-bold mb-2">Core</h2>
          <pre className="bg-gray-100 p-4 rounded whitespace-pre-wrap h-[calc(100%-40px)]">
            <Editor
              height="100%"
              value={core}
              onChange={(value) => setCode(value || "")}
              options={{
                fontSize: 14,
                minimap: { enabled: false },
                automaticLayout: true,
              }}
            />
          </pre>
        </div>

        <div className="h-[20%] overflow-auto p-4 border-t border-gray-300">
          <h2 className="text-sm font-bold mb-1">Stdout</h2>
          <pre className="bg-gray-100 p-2 rounded whitespace-pre-wrap text-sm h-[calc(100%-30px)]">
            {result}
          </pre>
        </div>
      </div>
    </div>
  );
}

export default App;