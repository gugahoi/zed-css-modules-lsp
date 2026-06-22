import styles from "./Foo.module.css";

// Put the cursor on `bar` (or `bazQux`) below and run "editor: go to definition"
// (f12 / cmd-click). It should jump to the matching class in Foo.module.css.
export function Foo() {
  return (
    <div className={styles.bar}>
      <span className={styles.bazQux}>hello</span>
    </div>
  );
}
