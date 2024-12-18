"use client";

// import Image from "next/image";
import styles from "./page.module.css";
import { useState } from "react";
import init, { execute } from "rust-calc";

export default function Home() {
  let wasm_init = false;
  const [value, setValue] = useState("");
  const [items, setItems] = useState<string[]>([]);
  const [modal, showModal] = useState(false);
  const [modalContent, setModalContent] = useState("");
  let modalTimeout: NodeJS.Timeout;
  const handleInput = async () => {
    if (!wasm_init) {
      wasm_init = true;
      await init();
    }
    if (value === "") {
      return;
    }
    try {
      const result = execute(value);
      const v = [];
      let no_operators = true;
      for (const c of value) {
        if (c === "+" || c === "-" || c === "*" || c === "/") {
          v.push(` ${c} `);
          no_operators = false;
        }
        else {
          v.push(c);
        }
      }
      const expr = v.join("");
      const res = no_operators ? `${expr}` : `${expr} = ${result}`;
      setItems([...items, res]);
    }
    catch (err) {
      clearTimeout(modalTimeout);
      setModalContent((err as Error).toString());
      showModal(true);

      modalTimeout = setTimeout(() => {
        showModal(false);
      }, 3000);
    }
  };

  return (
    <>
      <div className={styles.history}>
        <ul>
          {items.map((item, index) => (
            <li key={index}>{item}</li>
          ))}
        </ul>
      </div>
      <div className={styles.input}>
        <input
          placeholder="Enter expression"
          type="text"
          value={value}
          onChange={e => {
            if (e.target.value.match(/[^0-9\+\-\*\/\(\)\.]+/g)) {
              return;
            }
            setValue(e.target.value);
          }}
          onKeyDown={e => {
            if (e.key === "Enter") {
              handleInput();
            }
          }} />
        <button className={styles.submitButton} onClick={handleInput}></button>
      </div>
      <div className={styles.modal + " " + (modal ? styles.showModal : "")}>{modalContent}</div >
    </>
  );
}
