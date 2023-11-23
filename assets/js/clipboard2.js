import Emitter from 'tiny-emitter';
import listen from 'good-listener';
import select from 'select';

/**
 * Create fake copy action wrapper using a fake element.
 * @param {String} target
 * @param {Object} options
 * @return {String}
 */
const fakeCopyAction = (value, options) => {
  const fakeElement = createFakeElement(value);
  options.container.appendChild(fakeElement);
  const selectedText = select(fakeElement);
  command('copy');
  fakeElement.remove();

  return selectedText;
};

/**
 * Copy action wrapper.
 * @param {String|HTMLElement} target
 * @param {Object} options
 * @return {String}
 */
const ClipboardActionCopy = (
  target,
  options = { container: document.body }
) => {
  let selectedText = '';
  if (typeof target === 'string') {
    selectedText = fakeCopyAction(target, options);
  } else if (
    target instanceof HTMLInputElement &&
    !['text', 'search', 'url', 'tel', 'password'].includes(target?.type)
  ) {
    // If input type doesn't support `setSelectionRange`. Simulate it. https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange
    selectedText = fakeCopyAction(target.value, options);
  } else {
    selectedText = select(target);
    command('copy');
  }
  return selectedText;
};

/**
 * Cut action wrapper.
 * @param {String|HTMLElement} target
 * @return {String}
 */
const ClipboardActionCut = (target) => {
  const selectedText = select(target);
  command('cut');
  return selectedText;
};


/**
 * Inner function which performs selection from either `text` or `target`
 * properties and then executes copy or cut operations.
 * @param {Object} options
 */
const ClipboardActionDefault = (options = {}) => {
  // Defines base properties passed from constructor.
  const { action = 'copy', container, target, text } = options;

  // Sets the `action` to be performed which can be either 'copy' or 'cut'.
  if (action !== 'copy' && action !== 'cut') {
    throw new Error('Invalid "action" value, use either "copy" or "cut"');
  }

  // Sets the `target` property using an element that will be have its content copied.
  if (target !== undefined) {
    if (target && typeof target === 'object' && target.nodeType === 1) {
      if (action === 'copy' && target.hasAttribute('disabled')) {
        throw new Error(
          'Invalid "target" attribute. Please use "readonly" instead of "disabled" attribute'
        );
      }

      if (
        action === 'cut' &&
        (target.hasAttribute('readonly') || target.hasAttribute('disabled'))
      ) {
        throw new Error(
          'Invalid "target" attribute. You can\'t cut text from elements with "readonly" or "disabled" attributes'
        );
      }
    } else {
      throw new Error('Invalid "target" value, use a valid Element');
    }
  }

  // Define selection strategy based on `text` property.
  if (text) {
    return ClipboardActionCopy(text, { container });
  }

  // Defines which selection strategy based on `target` property.
  if (target) {
    return action === 'cut'
      ? ClipboardActionCut(target)
      : ClipboardActionCopy(target, { container });
  }
};

/**
 * Creates a fake textarea element with a value.
 * @param {String} value
 * @return {HTMLElement}
 */
export function createFakeElement(value) {
  const isRTL = document.documentElement.getAttribute('dir') === 'rtl';
  const fakeElement = document.createElement('textarea');
  // Prevent zooming on iOS
  fakeElement.style.fontSize = '12pt';
  // Reset box model
  fakeElement.style.border = '0';
  fakeElement.style.padding = '0';
  fakeElement.style.margin = '0';
  // Move element out of screen horizontally
  fakeElement.style.position = 'absolute';
  fakeElement.style[isRTL ? 'right' : 'left'] = '-9999px';
  // Move element to the same position vertically
  let yPosition = window.pageYOffset || document.documentElement.scrollTop;
  fakeElement.style.top = `${yPosition}px`;

  fakeElement.setAttribute('readonly', '');
  fakeElement.value = value;

  return fakeElement;
}


/**
 * Executes a given operation type.
 * @param {String} type
 * @return {Boolean}
 */
export function command(type) {
  try {
    return document.execCommand(type);
  } catch (err) {
    return false;
  }
}


/**
 * Helper function to retrieve attribute value.
 * @param {String} suffix
 * @param {Element} element
 */
function getAttributeValue(suffix, element) {
  const attribute = `data-clipboard-${suffix}`;

  if (!element.hasAttribute(attribute)) {
    return;
  }

  return element.getAttribute(attribute);
}

/**
 * Base class which takes one or more elements, adds event listeners to them,
 * and instantiates a new `ClipboardAction` on each click.
 */
export class Clipboard extends Emitter {
  /**
   * @param {String|HTMLElement|HTMLCollection|NodeList} trigger
   * @param {Object} options
   */
  constructor(trigger, options) {
    super();

    this.resolveOptions(options);
    this.listenClick(trigger);
  }

  /**
   * Defines if attributes would be resolved using internal setter functions
   * or custom functions that were passed in the constructor.
   * @param {Object} options
   */
  resolveOptions(options = {}) {
    this.action =
      typeof options.action === 'function'
        ? options.action
        : this.defaultAction;
    this.target =
      typeof options.target === 'function'
        ? options.target
        : this.defaultTarget;
    this.text =
      typeof options.text === 'function' ? options.text : this.defaultText;
    this.container =
      typeof options.container === 'object' ? options.container : document.body;
  }

  /**
   * Adds a click event listener to the passed trigger.
   * @param {String|HTMLElement|HTMLCollection|NodeList} trigger
   */
  listenClick(trigger) {
    this.listener = listen(trigger, 'click', (e) => this.onClick(e));
  }

  /**
   * Defines a new `ClipboardAction` on each click event.
   * @param {Event} e
   */
  onClick(e) {
    const trigger = e.delegateTarget || e.currentTarget;
    const action = this.action(trigger) || 'copy';
    const text = ClipboardActionDefault({
      action,
      container: this.container,
      target: this.target(trigger),
      text: this.text(trigger),
    });

    // Fires an event based on the copy operation result.
    this.emit(text ? 'success' : 'error', {
      action,
      text,
      trigger,
      clearSelection() {
        if (trigger) {
          trigger.focus();
        }
        window.getSelection().removeAllRanges();
      },
    });
  }

  /**
   * Default `action` lookup function.
   * @param {Element} trigger
   */
  defaultAction(trigger) {
    return getAttributeValue('action', trigger);
  }

  /**
   * Default `target` lookup function.
   * @param {Element} trigger
   */
  defaultTarget(trigger) {
    const selector = getAttributeValue('target', trigger);

    if (selector) {
      return document.querySelector(selector);
    }
  }

  /**
   * Allow fire programmatically a copy action
   * @param {String|HTMLElement} target
   * @param {Object} options
   * @returns Text copied.
   */
  static copy(target, options = { container: document.body }) {
    return ClipboardActionCopy(target, options);
  }

  /**
   * Allow fire programmatically a cut action
   * @param {String|HTMLElement} target
   * @returns Text cutted.
   */
  static cut(target) {
    return ClipboardActionCut(target);
  }

  /**
   * Returns the support of the given action, or all actions if no action is
   * given.
   * @param {String} [action]
   */
  static isSupported(action = ['copy', 'cut']) {
    const actions = typeof action === 'string' ? [action] : action;
    let support = !!document.queryCommandSupported;

    actions.forEach((action) => {
      support = support && !!document.queryCommandSupported(action);
    });

    return support;
  }

  /**
   * Default `text` lookup function.
   * @param {Element} trigger
   */
  defaultText(trigger) {
    return getAttributeValue('text', trigger);
  }

  /**
   * Destroy lifecycle.
   */
  destroy() {
    this.listener.destroy();
  }
}

export default Clipboard;