(() => {
  const L = globalThis.__lt;
  if (!L || L.act) return;
  const { S, get, ref, nameOf, clean, trunc, q, describe } = L;
  const { visible, text, table, viewSize, frameBox, settle, scrollInfo, scrollBy, visibleText, peek } = L.read;
  const REJECT = /(reject|decline|deny|refuse|necessary only|only necessary|essential only|ablehnen|nur notwendige|nur erforderliche|verweigern)/i;
  const ACCEPT = /(accept all|accept|agree|allow all|got it|okay|^ok$|akzeptieren|zustimmen|einverstanden|alle erlauben|verstanden)/i;
  const IMPLICIT = /^(text|search|url|tel|email|password|date|month|week|time|datetime-local|number)$/;
  const FIELD = 'input:not([type="hidden"]):not([type="button"]):not([type="submit"]):not([type="reset"]):not([type="image"]),textarea,select,[contenteditable=""],[contenteditable="true"],[role="textbox"],[role="combobox"],[role="searchbox"],[role="checkbox"],[role="radio"],[role="switch"]';

  function score(name, ql) {
    const n = name.toLowerCase();
    if (!n) return 0;
    if (n === ql) return 100;
    if (n.startsWith(ql)) return 60;
    if (new RegExp("(^|\\W)" + ql.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "(\\W|$)").test(n)) return 45;
    if (n.includes(ql)) return 20;
    return 0;
  }

  function rank(cands, ql, opts) {
    const list = [];
    for (const c of cands) {
      let s = score(c.name, ql);
      if (!s && c.el.value && typeof c.el.value === "string") s = score(clean(c.el.value), ql) / 2;
      if (!s) continue;
      if (c.inView) s += 10;
      if (c.role === "button" || c.role === "link") s += 3;
      if (opts && opts.role && c.role !== opts.role) continue;
      list.push({ c, s });
    }
    return list.sort((a, b) => b.s - a.s);
  }

  function pick(list, opts) {
    const top = list[0].s;
    const same = list.filter((x) => x.s === top && x.c.name.toLowerCase() === list[0].c.name.toLowerCase());
    if (same.length > 1 && !(opts && opts.first)) {
      return { ambiguous: same.slice(0, 4).map((x) => L.line(x.c.el, x.c.role, x.c.name)) };
    }
    const b = list[0].c;
    return { ref: ref(b.el), name: b.name, role: b.role };
  }

  function byHint(h) {
    const v = CSS.escape(h);
    const sel = ["data-e2e", "data-testid", "data-test", "data-qa", "data-cy", "name"].map((a) => "[" + a + '="' + v + '"]').concat("#" + v).join(",");
    let list = [];
    try {
      list = [...document.querySelectorAll(sel)].filter(visible);
    } catch (e) {}
    const el = list.find((x) => x.getBoundingClientRect().top < innerHeight && x.getBoundingClientRect().bottom > 0) || list[0];
    return el ? { ref: ref(el), name: clean(nameOf(el)), role: L.roleOf(el) } : { none: true };
  }

  function find(query, opts) {
    if (/^#[\w:.-]+$/.test(query.trim())) return byHint(query.trim().slice(1));
    const ql = clean(query).toLowerCase();
    if (!ql) return { none: true };
    const needle = ql.replace(/\s+/g, "");
    const quick = rank(L.candidates(null, needle, true), ql, opts);
    if (quick.length && quick[0].s >= 110) return pick(quick, opts);
    const list = rank(L.candidates(null, needle), ql, opts);
    if (list.length) return pick(list, opts);
    const tw = document.createTreeWalker(document.body || document.documentElement, NodeFilter.SHOW_TEXT);
    let t;
    while ((t = tw.nextNode())) {
      if (!t.data || !t.data.toLowerCase().includes(ql)) continue;
      const host = t.parentElement;
      if (!host || !visible(host)) continue;
      const target = host.closest(L.CONTROL) || host;
      return { ref: ref(target), name: trunc(clean(target.innerText || t.data), 60), role: "text" };
    }
    return { none: true };
  }

  function focusFor(r, append) {
    const el = get(r);
    if (!el) return { gone: true };
    el.focus({ preventScroll: false });
    const t = el.tagName;
    if (!append) {
      if ((t === "INPUT" || t === "TEXTAREA") && typeof el.select === "function") {
        try {
          el.select();
        } catch (e) {}
      } else if (el.isContentEditable) {
        const range = document.createRange();
        range.selectNodeContents(el);
        const sel = getSelection();
        sel.removeAllRanges();
        sel.addRange(range);
      }
    }
    return { ok: true, focused: document.activeElement === el || el.contains(document.activeElement) };
  }

  function valueOf(r) {
    const el = get(r);
    if (!el) return { gone: true };
    const v = el.isContentEditable ? el.innerText : el.value;
    return { value: typeof v === "string" ? v : "" };
  }

  function setValue(r, text) {
    const el = get(r);
    if (!el) return { gone: true };
    if (el.isContentEditable) {
      el.innerText = text;
    } else {
      const proto = el.tagName === "TEXTAREA" ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
      const setter = Object.getOwnPropertyDescriptor(proto, "value").set;
      setter.call(el, text);
    }
    el.dispatchEvent(new Event("input", { bubbles: true }));
    el.dispatchEvent(new Event("change", { bubbles: true }));
    return { ok: true };
  }

  function selectOpt(r, option) {
    const el = get(r);
    if (!el) return { gone: true };
    if (el.tagName !== "SELECT") return { notSelect: true };
    const ol = clean(option).toLowerCase();
    const opts = [...el.options];
    const hit = opts.find((o) => clean(o.text).toLowerCase() === ol) || opts.find((o) => o.value.toLowerCase() === ol) || opts.find((o) => clean(o.text).toLowerCase().includes(ol));
    if (!hit) return { missing: opts.slice(0, 12).map((o) => clean(o.text)) };
    el.value = hit.value;
    el.dispatchEvent(new Event("input", { bubbles: true }));
    el.dispatchEvent(new Event("change", { bubbles: true }));
    return { chosen: clean(hit.text) };
  }

  function checked(r) {
    const el = get(r);
    if (!el) return { gone: true };
    const a = el.getAttribute("aria-checked");
    return { on: el.checked === true || a === "true" };
  }

  function fields(labels) {
    const pool = [...document.querySelectorAll(FIELD)].filter(visible);
    const used = new Set();
    return labels.map((label) => {
      const ll = clean(label).toLowerCase();
      let best = null, bs = 0;
      for (const el of pool) {
        if (used.has(el)) continue;
        let s = score(nameOf(el), ll);
        const attrs = [el.getAttribute("name"), el.id, el.getAttribute("autocomplete")].filter(Boolean).map((x) => x.toLowerCase());
        if (attrs.includes(ll)) s = Math.max(s, 80);
        if (s > bs) {
          bs = s;
          best = el;
        }
      }
      if (!best) return { label, none: true };
      used.add(best);
      const role = L.roleOf(best);
      return { label, ref: ref(best), role, tag: best.tagName, type: (best.type || "").toLowerCase() };
    });
  }

  function submitOf(r) {
    const el = get(r);
    if (!el) return { gone: true };
    const form = el.form || el.closest("form");
    if (!form) return { none: true };
    const btn = form.querySelector('button[type="submit"],input[type="submit"],button:not([type])');
    if (!btn || !visible(btn)) return { form: true, none: true };
    return { ref: ref(btn), name: nameOf(btn) };
  }

  function test(spec) {
    if (spec.ref) {
      const el = get(spec.ref);
      return !!el && visible(el);
    }
    if (spec.css) {
      const el = document.querySelector(spec.css);
      return !!el && visible(el);
    }
    if (spec.text) {
      const t = spec.text.toLowerCase();
      if ((document.body ? document.body.innerText : "").toLowerCase().includes(t)) return true;
      for (const el of document.querySelectorAll("[aria-label],[title],img[alt],input[value],input[placeholder]")) {
        const n = el.getAttribute("aria-label") || el.getAttribute("title") || el.getAttribute("alt") || el.value || el.getAttribute("placeholder") || "";
        if (n.toLowerCase().includes(t) && visible(el)) return true;
      }
    }
    return false;
  }

  function waitFor(spec, timeout) {
    return new Promise((resolve) => {
      const t0 = Date.now();
      const tick = () => {
        const hit = test(spec);
        if (spec.gone ? !hit : hit) return resolve({ ok: true, ms: Date.now() - t0 });
        if (Date.now() - t0 >= timeout) return resolve({ ok: false, ms: Date.now() - t0 });
        setTimeout(tick, 100);
      };
      tick();
    });
  }

  function marks(on) {
    const old = document.querySelector("lt-pointer[data-marks]");
    if (old) old.remove();
    if (!on) return { n: 0 };
    const host = document.createElement("lt-pointer");
    host.setAttribute("data-marks", "");
    host.style.cssText = "all:initial;position:fixed;left:0;top:0;width:0;height:0;z-index:2147483647;pointer-events:none";
    const sh = host.attachShadow({ mode: "closed" });
    let n = 0;
    for (const r of S.last) {
      const el = get(r);
      if (!el) continue;
      const b = el.getBoundingClientRect();
      const off = L.frameOffset(el.ownerDocument);
      const x = b.left + off.x, y = b.top + off.y;
      if (b.width < 2 || b.height < 2 || y + b.height <= 0 || x + b.width <= 0 || y >= innerHeight || x >= innerWidth) continue;
      const tag = document.createElement("span");
      tag.textContent = r.replace(/^e/, "");
      tag.style.cssText = "position:fixed;left:" + Math.max(0, Math.round(x) - 2) + "px;top:" + Math.max(0, Math.round(y) - 2) + "px;font:bold 11px/13px Arial,sans-serif;color:#fff;background:#ff8a00;border:1px solid #fff;border-radius:3px;padding:0 3px;box-shadow:0 1px 2px rgba(0,0,0,.6)";
      sh.appendChild(tag);
      n++;
    }
    document.documentElement.appendChild(host);
    return { n };
  }

  function pointer(x, y) {
    let host = document.querySelector("lt-pointer:not([data-marks])");
    if (!host) {
      host = document.createElement("lt-pointer");
      const sh = host.attachShadow({ mode: "closed" });
      const ns = "http://www.w3.org/2000/svg";
      const svg = document.createElementNS(ns, "svg");
      svg.setAttribute("width", "22");
      svg.setAttribute("height", "28");
      svg.setAttribute("viewBox", "0 0 22 28");
      const path = document.createElementNS(ns, "path");
      path.setAttribute("d", "M1 1 L1 22 L7 16 L11 26 L15 24 L11 15 L19 15 Z");
      path.setAttribute("fill", "#ff8a00");
      path.setAttribute("stroke", "#ffffff");
      path.setAttribute("stroke-width", "1.5");
      svg.appendChild(path);
      svg.style.cssText = "display:block;filter:drop-shadow(0 1px 2px rgba(0,0,0,.55))";
      sh.appendChild(svg);
      host.style.cssText = "all:initial;position:fixed;left:0;top:0;z-index:2147483647;pointer-events:none;transition:transform .16s ease-out,opacity .35s;opacity:0";
      document.documentElement.appendChild(host);
    }
    host.style.transform = "translate(" + (x - 1) + "px," + (y - 1) + "px)";
    host.style.opacity = "1";
    clearTimeout(host.__t);
    host.__t = setTimeout(() => {
      host.style.opacity = "0";
    }, 1500);
    return { ok: true };
  }

  function buttonsIn(root) {
    return [...root.querySelectorAll('button,a,[role="button"],input[type="button"],input[type="submit"]')];
  }

  function dismiss() {
    const sel = '#onetrust-banner-sdk,#didomi-host,#usercentrics-root,#CybotCookiebotDialog,.cc-window,[id*="cookie" i],[class*="cookie" i],[id*="consent" i],[class*="consent" i],[id*="gdpr" i],[class*="gdpr" i],[aria-label*="cookie" i],[aria-label*="consent" i]';
    const banners = [...document.querySelectorAll(sel)].filter((b) => visible(b) || (b.shadowRoot && b.shadowRoot.childElementCount));
    for (const re of [REJECT, ACCEPT]) {
      for (const b of banners) {
        const pool = buttonsIn(b).concat(b.shadowRoot ? buttonsIn(b.shadowRoot) : []);
        for (const btn of pool) {
          const n = nameOf(btn);
          if (re.test(n) && visible(btn)) {
            btn.click();
            return { clicked: trunc(n, 40), kind: re === REJECT ? "rejected" : "accepted" };
          }
        }
      }
    }
    return { none: true };
  }

  function fileInput(el) {
    const isFile = (x) => x && x.matches && x.matches("input[type=file]");
    if (isFile(el)) return el;
    const label = el.control !== undefined ? el : el.closest("label");
    if (label && isFile(label.control)) return label.control;
    const doc = el.ownerDocument || document;
    const own = el.htmlFor || el.getAttribute("for");
    if (own && isFile(doc.getElementById(own))) return doc.getElementById(own);
    for (let p = el, i = 0; p && i < 4 && p !== doc.body; p = p.parentElement, i++) {
      const inp = p.querySelector("input[type=file]");
      if (inp) return inp;
    }
    const all = doc.querySelectorAll("input[type=file]");
    return all.length === 1 ? all[0] : null;
  }

  function mark(r, id, file) {
    const found = get(r);
    if (!found) return { gone: true };
    const el = file ? fileInput(found) : found;
    if (!el) return { nofile: true };
    el.setAttribute("data-lt-mark", id);
    return { ok: true, tag: el.tagName, type: (el.type || "").toLowerCase() };
  }

  function unmark() {
    for (const el of document.querySelectorAll("[data-lt-mark]")) el.removeAttribute("data-lt-mark");
    return { ok: true };
  }

  function searchField() {
    const SEARCH = /search|such|find|finde|recherche|buscar|cerca|zoek/i;
    let best = null, top = 0;
    for (const el of document.querySelectorAll('input:not([type]),input[type="search"],input[type="text"],[role="searchbox"],[role="combobox"],textarea')) {
      if (!visible(el) || el.disabled || el.readOnly) continue;
      const hay = [el.name, el.id, el.getAttribute("placeholder"), el.getAttribute("aria-label"), el.getAttribute("title"), nameOf(el)].join(" ");
      let s = 0;
      if (el.type === "search" || el.getAttribute("role") === "searchbox") s += 5;
      if (/^(q|query|search|s|k|keyword|search_query)$/i.test(el.name || "")) s += 3;
      if (SEARCH.test(hay)) s += 4;
      if (el.closest('form[role="search"],[role="search"]')) s += 3;
      if (s > top) {
        best = el;
        top = s;
      }
    }
    if (best) return { ref: ref(best) };
    for (const el of document.querySelectorAll('button,[role="button"],a[href]')) {
      if (visible(el) && SEARCH.test(nameOf(el) || "")) return { button: ref(el) };
    }
    return { none: true };
  }

  function lineOf(r) {
    const el = get(r);
    return el ? L.line(el, L.roleOf(el), nameOf(el)) : null;
  }

  function info(r) {
    const el = get(r);
    if (!el) return { gone: true };
    return { name: clean(nameOf(el)), role: L.roleOf(el), tag: el.tagName, desc: describe(el), hint: L.hintOf(el) };
  }

  function frameHit(x, y) {
    const el = document.elementFromPoint(x, y);
    if (!el) return { covered: "the edge of the window" };
    return { covered: el.tagName === "IFRAME" || el.tagName === "FRAME" ? null : describe(el) };
  }

  function domClick(r) {
    const el = get(r);
    if (!el) return { gone: true };
    el.click();
    return { ok: true };
  }

  function armClick() {
    S.clickedAt = 0;
    document.addEventListener("click", () => {
      S.clickedAt = Date.now();
    }, { capture: true, once: true });
    return { ok: true };
  }

  function clickSeen() {
    return { seen: !!S.clickedAt };
  }

  function armKey() {
    S.keyAt = 0;
    window.addEventListener("keydown", () => {
      S.keyAt = Date.now();
    }, { capture: true, once: true });
    return { ok: true };
  }

  function keySeen() {
    return { seen: S.keyAt === undefined ? null : !!S.keyAt };
  }

  function domEnter(r) {
    const el = get(r);
    if (!el) return { gone: true };
    const form = el.tagName === "INPUT" ? el.form : null;
    if (!form) {
      for (const type of ["keydown", "keypress", "keyup"]) {
        el.dispatchEvent(new KeyboardEvent(type, { key: "Enter", code: "Enter", keyCode: 13, which: 13, bubbles: true, cancelable: true }));
      }
      return { ok: true };
    }
    const els = [...form.elements];
    const btn = els.find((x) => x.type === "submit" && !x.disabled);
    if (btn) btn.click();
    else if (els.filter((x) => x.tagName === "INPUT" && IMPLICIT.test(x.type)).length <= 1) form.requestSubmit();
    else return { none: true };
    return { ok: true };
  }

  function focusedRef() {
    let el = document.activeElement;
    while (el && el.shadowRoot && el.shadowRoot.activeElement) el = el.shadowRoot.activeElement;
    if (!el || el === document.body || el === document.documentElement) return { none: true };
    if (el.tagName === "IFRAME" || el.tagName === "FRAME") return { frame: true };
    return { ref: ref(el) };
  }

  L.act = { find, focusedRef, focusFor, valueOf, setValue, selectOpt, checked, fields, submitOf, text, table, waitFor, settle, pointer, marks, dismiss, mark, unmark, scrollInfo, scrollBy, visibleText, peek, searchField, lineOf, info, viewSize, frameBox, frameHit, domClick, armClick, clickSeen, armKey, keySeen, domEnter };
})();
