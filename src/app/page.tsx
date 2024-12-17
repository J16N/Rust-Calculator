"use client";

// import Image from "next/image";
import styles from "./page.module.css";
import { useState } from "react";
// import init, { greet } from "rust-calc";

export default function Home() {
  // (async function () {
  //   await init();
  //   greet();
  // })();

  const [value, setValue] = useState("");

  return (
    <div className={styles.page}>
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
      <button className={styles.submitButton}></button>
    </div>
  );
}
