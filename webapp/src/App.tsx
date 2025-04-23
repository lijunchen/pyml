import Editor, { useMonaco } from '@monaco-editor/react';
import { useEffect, useState } from 'react';
import { execute, compile_to_core, hover, get_cst, get_ast, get_tast } from 'wasm-app';
import * as monacoEditor from 'monaco-editor/esm/vs/editor/editor.api';

const demos: Record<string, string> = {};

const loadDemos = async () => {
  const modules = import.meta.glob('../../crates/compiler/src/tests/examples/*.src', { query: '?raw', import: 'default' });
  for (const path in modules) {
    const name = path.split('/').pop()?.replace('.src', '') || 'unknown';
    demos[name] = await modules[path]() as string;
  }
};

function App() {
  const monaco = useMonaco();
  const [code, setCode] = useState("");
  const [result, setResult] = useState("");
  const [core, setCore] = useState("");
  const [selectedDemo, setSelectedDemo] = useState("");
  const [viewMode, setViewMode] = useState("core");

  useEffect(() => {
    loadDemos().then(() => {
      const defaultDemo = Object.keys(demos)[0];
      setSelectedDemo(defaultDemo);
      setCode(demos[defaultDemo]);
    });
  }, []);

  useEffect(() => {
    if (monaco) {
      monaco.languages.register({ id: 'simple' });

      monaco.languages.setMonarchTokensProvider('simple', {
        keywords: ['fn', 'let', 'in'],
        tokenizer: {
          root: [
            [/\b(fn|enum|trait|impl|for|match|if|else|let|in|return|true|false|Unit|Bool|Int)\b/, "keyword"],
            [/\b[A-Z][a-zA-Z0-9_]*\b/, "type"],
            [/\b\d+\b/, "number"],
            [/[a-zA-Z_]\w*(?=\s*\()/, "function"],
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
          { token: 'type', foreground: '216C86' },
          { token: 'number', foreground: '09885A' },
          { token: 'identifier', foreground: '001080' },
          { token: 'string', foreground: 'A31515' },
          { token: 'function', foreground: '654D1D' },
        ],
        colors: {},
      });

      monaco.languages.registerHoverProvider('simple', {
        provideHover: async (
          model: monacoEditor.editor.ITextModel, 
          position: monacoEditor.Position
        ): Promise<monacoEditor.languages.Hover | null> => {
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

      monaco.editor.setTheme('simpleTheme');
    }
  }, [monaco]);

  useEffect(() => {
    try {
      if (viewMode === "core") setCore(compile_to_core(code));
      else if (viewMode === "cst") setCore(get_cst(code));
      else if (viewMode === "ast") setCore(get_ast(code));
      else if (viewMode === "tast") setCore(get_tast(code));
      setResult(execute(code));
    } catch (error) {
      console.error(error);
    }
  }, [code, viewMode]);

  const handleDemoChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const demoName = e.target.value;
    setSelectedDemo(demoName);
    setCode(demos[demoName]);
  };

  const handleViewModeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setViewMode(e.target.value);
    try {
      if (e.target.value === "cst") setCore(get_cst(code));
      else if (e.target.value === "ast") setCore(get_ast(code));
      else if (e.target.value === "tast") setCore(get_tast(code));
      else setCore(compile_to_core(code));
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="h-screen flex flex-col">
      <div className="bg-gray-100 p-2 flex items-center">
        <label className="mr-2 font-medium">Select Demo:</label>
        <select 
          value={selectedDemo}
          onChange={handleDemoChange}
          className="border rounded p-1 mr-4"
        >
          {Object.keys(demos).map(demo => (
            <option key={demo} value={demo}>
              {demo.replace(/_/g, ' ')}
            </option>
          ))}
        </select>
        <label className="mr-2 font-medium">View Mode:</label>
        <select 
          value={viewMode}
          onChange={handleViewModeChange}
          className="border rounded p-1"
        >
          <option value="cst">CST</option>
          <option value="ast">AST</option>
          <option value="tast">TAST</option>
          <option value="core">Core</option>
        </select>
      </div>
      
      <div className="flex flex-1">
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
            <h2 className="text-xl font-bold mb-2">{viewMode.toUpperCase()}</h2>
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
    </div>
  );
}

export default App;