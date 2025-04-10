import './App.css'

import { execute } from 'wasm-app'

function App() {
  let result = execute("let a = print_int(123) in a");
  console.log(result);

  return (
    <>
    </>
  )
}

export default App
