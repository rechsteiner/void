import { keymap, EditorView } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { lineNumbers } from "@codemirror/gutter";
import { history, historyKeymap } from "@codemirror/history";
import { defaultKeymap } from "@codemirror/commands";

const EDITOR_DOCUMENT = "editor-document";

/// This class sets up a code editor inside the given DOM element. You can read
/// the current editor state using the `document` property, or listen to changes
/// by passing in a `onChange` function to the options. The changes made inside
/// the editor will be cached using local storage.
export class Editor {
  constructor(element, options) {
    const updateListener = () => {
      return EditorView.updateListener.of((view) => {
        if (view.docChanged && options.onChange) {
          const document = this.document;
          window.localStorage.setItem(EDITOR_DOCUMENT, document);
          options.onChange(document);
        }
      });
    };

    this.editorView = new EditorView({
      state: EditorState.create({
        doc: window.localStorage.getItem(EDITOR_DOCUMENT),
        extensions: [
          updateListener(),
          lineNumbers(),
          history(),
          keymap.of([...defaultKeymap, ...historyKeymap]),
        ],
      }),
      parent: element,
      // Provide a custom dispatch implementation that takes all editor changes
      // and uppercases them. This might be possible do on a language level, but
      // will look into that when we add our own language plugin.
      dispatch: (transaction) => {
        transaction.changes.inserted.forEach((insertion) => {
          if (insertion.text !== undefined) {
            insertion.text = insertion.text.map((text) => {
              return text.toUpperCase();
            });
          }
        });
        this.editorView.update([transaction]);
      },
    });
  }

  get document() {
    return this.editorView.state.doc.toString();
  }
}
