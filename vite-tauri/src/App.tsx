import { useState, useEffect, useRef } from 'react'
import logo from './logo.svg'
import './App.css'

import { invoke } from '@tauri-apps/api'
import { listen, Event, UnlistenFn } from '@tauri-apps/api/event'

function App() {
  const [count, setCount] = useState(0)
  const [connect, setConnect] = useState(false)
  const unListenFn = useRef<UnlistenFn | null>(null)

  useEffect(() => {
    (async () => {
      unListenFn.current = await listen("keep-alive", (_: Event<any>) => {
        setConnect(true)
        setTimeout(() => {
          setConnect(false)
        }, 500)
      })
    })()
    return () => {
      unListenFn.current && unListenFn.current()
    }
  }, [])

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={async () => {
            const result: number = await invoke("count", { n: 1 })
            setCount(result)
          }}>
            count is: {count}
          </button>
          <button type="button" onClick={async () => {
            const result: string = await invoke("hello") 
            alert(`rust say: ${result}`)
          }}>
            rust say hello
          </button>
        </p>
        <p>
          Edit <code>App.tsx</code> and save to test HMR updates.
        </p>
        <p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          {' | '}
          <span
            className="tokio"
            style={{
              background: connect ? 'chartreuse' : 'red'
            }}
          >
          </span>{' | '}
          <a
            className="App-link"
            href="https://vitejs.dev/guide/features.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Vite Docs
          </a>
        </p>
      </header>
    </div>
  )
}

export default App
