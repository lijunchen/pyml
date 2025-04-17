import Editor, { useMonaco } from '@monaco-editor/react';
import { useEffect, useState } from 'react';
import { execute, compile_to_core, hover } from 'wasm-app';

function App() {
  const monaco = useMonaco();
  const [code, setCode] = useState("fn main() {\n  let a = println(int_to_string(123)) in\n  a\n}\n");
  const [result, setResult] = useState("");
  const [core, setCore] = useState("");

  useEffect(() => {
    if (monaco) {
      monaco.languages.register({ id: 'simple' });

      monaco.languages.setMonarchTokensProvider('simple', {
        keywords: ['fn', 'let', 'in'],
        tokenizer: {
          root: [
            [/\b(fn|enum|match|if|else|let|in|return|true|false|Unit|Bool|Int)\b/, "keyword"],
            [/\b\d+\b/, "number"],
            [/[a-zA-Z_]\w*/, "identifier"],
            [/[{}()\[\]]/, "@brackets"],
            [/[;,.]/, "delimiter"],
            [/".*?"/, "string"],
            [/\/\/.*/, "comment"],
          ],
        },
      });

      monaco.editor.defineTheme('simpleTheme', {
        base: 'vs',
        inherit: true,
        rules: [
          { token: 'keyword', foreground: '0000FF' },
          { token: 'number', foreground: '09885A' },
          { token: 'identifier', foreground: '001080' },
          { token: 'string', foreground: 'A31515' },
        ],
        colors: {},
      });

      monaco.languages.registerHoverProvider('simple', {
        provideHover: async (model, position) => {
          const line = position.lineNumber - 1;
          const col = position.column - 1;
          const content = model.getValue();
          const hoverText = hover(content, line, col);

          if (hoverText) {
            return {
              contents: [{ value: `\`\`\`simple\n${hoverText}\n\`\`\`` }],
            };
          }
          return null;
        },
      });
    }
  }, [monaco]);

  useEffect(() => {
    try {
      setResult(execute(code));
      setCore(compile_to_core(code));
    } catch (error) {
      console.error(error);
    }
  }, [code]);

  return (
    <div className="h-screen flex">
      <div className="w-1/2 border-r border-gray-300 flex flex-col">
        <Editor
          height="100%"
          language="simple"
          theme="simpleTheme"
          value={code}
          onChange={(value) => setCode(value || "")}
          options={{ fontSize: 14, minimap: { enabled: false }, automaticLayout: true }}
        />
      </div>

      <div className="w-1/2 flex flex-col h-full">
        <div className="flex-grow h-[80%] overflow-auto p-4">
          <h2 className="text-xl font-bold mb-2">Core</h2>
          <Editor
            height="100%"
            language="plaintext"
            value={core}
            options={{ fontSize: 14, minimap: { enabled: false }, readOnly: true }}
          />
        </div>

        <div className="h-[20%] overflow-auto p-4 border-t border-gray-300">
          <h2 className="text-sm font-bold mb-1">Stdout</h2>
          <pre className="bg-gray-100 p-2 rounded whitespace-pre-wrap text-sm">
            {result}
          </pre>
        </div>
      </div>
    </div>
  );
}

export default App;