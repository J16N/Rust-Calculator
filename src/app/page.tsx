"use client";

// import Image from "next/image";
import styles from "./page.module.css";
import { useState } from "react";
import init, { execute } from "rust-calc";

export default function Home() {
  let wasm_init = false;
  const [value, setValue] = useState("");

  return (
    <div className={styles.page}>
      <div className={styles.history}></div>
      <input
        placeholder="Enter expression"
        type="text"
        value={value}
        onChange={e => {
          if (e.target.value.match(/[^0-9\+\-\*\/\(\)\.]+/g)) {
            return;
          }
          setValue(e.target.value);
        }} />
      <button className={styles.submitButton} onClick={async () => {
        if (!wasm_init) {
          wasm_init = true;
          await init();
        }
        if (value === "") {
          return;
        }
        try {
          const result = execute(value);
          setValue(result);
        }
        catch (e) {
          console.error(e);
        }
      }}></button>
    </div>
  );
}
