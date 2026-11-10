(() => {
  const L = globalThis.__lt;
  if (!L || L.snap) return;
  const { S, get, ref, roleOf, nameOf, clean, trunc, q, covered, line, ITEM, LANDMARK, where, walk, flushAnimations } = L;

  const HIT_BUDGET_MS = 60;
  const WIDE_BUDGET_MS = 500;
  const WIDE_LINES = 12;

  function overlays(cut) {
    const out = [];
    for (const { el, off } of cut.layers) {
      const r = el.getBoundingClientRect();
      if (r.width < 8 || r.height < 8) continue;
      const x = r.left + off.x, y = r.top + off.y;
      if (y >= innerHeight || x >= innerWidth || y + r.height <= 0 || x + r.width <= 0) continue;
      if (!el.checkVisibility({ opacityProperty: true, visibilityProperty: true })) continue;
      out.push({ el, x, y, r: x + r.width, b: y + r.height });
    }
    return out;
  }

  function mayBeCovered(el, x, y, w, h, layers) {
    for (const o of layers) {
      if (o.x >= x + w || o.r <= x || o.y >= y + h || o.b <= y) continue;
      if (o.el === el || o.el.contains(el) || el.contains(o.el)) continue;
      return true;
    }
    return false;
  }

  function modalOf() {
    const list = document.querySelectorAll('dialog[open],[aria-modal="true"],[role="dialog"],[role="alertdialog"]');
    let best = null;
    for (const m of list) {
      if (!m.checkVisibility({ opacityProperty: true, visibilityProperty: true })) continue;
      const r = m.getBoundingClientRect();
      if (r.width < 80 || r.height < 40) continue;
      best = m;
    }
    return best;
  }

  function notes(frames) {
    const out = [];
    const cap = document.querySelector('iframe[src*="recaptcha"],iframe[src*="hcaptcha"],iframe[src*="challenges.cloudflare.com"],iframe[src*="arkoselabs"],#challenge-form,.cf-turnstile,#px-captcha');
    if (cap && cap.checkVisibility()) out.push("! captcha on page: ask the user to solve it");
    const ck = document.querySelector('#onetrust-banner-sdk,#didomi-host,#usercentrics-root,#CybotCookiebotDialog,.cc-window,[id*="cookie" i][role="dialog"],[class*="cookie-banner" i],[id*="cookie-banner" i],[class*="consent-banner" i],[id*="consent" i][role="dialog"]');
    if (ck && ck.checkVisibility({ opacityProperty: true, visibilityProperty: true })) out.push("! cookie banner: wisply dismiss");
    for (const f of frames.slice(0, 5)) {
      if (!f.checkVisibility()) continue;
      const r = f.getBoundingClientRect();
      if (r.width < 50 || r.height < 30) continue;
      let host = "";
      try {
        host = new URL(f.src, location.href).host;
      } catch (e) {}
      if (host) out.push("iframe " + host + " (cross-origin, wisply snap --frame " + host + ")");
    }
    return out;
  }

  function collectLines(opts) {
    const vw = innerWidth, vh = innerHeight;
    let scope = document.documentElement, modal = null;
    const notesOut = [];
    if (opts.scope) {
      const s = /^e\d+$/.test(opts.scope) ? get(opts.scope) : document.querySelector(opts.scope);
      if (!s) return { error: "scope " + opts.scope + " not found" };
      scope = s;
    } else if (!opts.all) {
      modal = modalOf();
      if (modal) {
        scope = modal;
        notesOut.push('modal "' + q(trunc(nameOf(modal), 50)) + '" open: page behind is hidden (press Escape to close)');
      }
    }
    flushAnimations();
    const items = [], frames = [];
    const cut = { below: 0, above: 0, layers: [], prune: !opts.all, images: !!opts.media, until: opts.until };
    walk(scope, { x: 0, y: 0 }, items, frames, cut);
    const layers = overlays(cut);
    let hitMs = 0;
    const filter = opts.filter ? opts.filter.toLowerCase().split("|").map((s) => s.trim()).filter(Boolean) : null;
    const esc = (f) => f.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const tiers = filter ? filter.map((f) => [new RegExp("(^|[^\\p{L}\\p{N}])" + esc(f) + "($|[^\\p{L}\\p{N}])", "u"), new RegExp("(^|[^\\p{L}\\p{N}])" + esc(f), "u"), f]) : null;
    const tierOf = (texts) => tiers.map(([w, s, f]) => Math.max(...texts.map((t) => (w.test(t) ? 3 : s.test(t) ? 2 : t.includes(f) ? 1 : 0))));
    const lines = [], seen = new Set();
    let below = 0, above = 0;
    const land = new Map();
    const groups = new Map();
    const collapseOk = !opts.all && !opts.scope && !filter;
    for (const it of items) {
      if (cut.until && performance.now() > cut.until) {
        cut.late = true;
        break;
      }
      const el = it.el;
      const r = el.getBoundingClientRect();
      if (r.width <= 1 || r.height <= 1) continue;
      const av = it.media && el.tagName !== "IMG";
      if (!it.heading && !av && !el.checkVisibility({ opacityProperty: true, visibilityProperty: true })) continue;
      if (!av && el.closest('[aria-hidden="true"]')) continue;
      const ax = r.left + it.off.x, ay = r.top + it.off.y;
      const pos = where(ax, ay, r.width, r.height);
      const inView = pos === "in";
      if (!inView && !opts.all) {
        if (!it.heading) {
          if (pos === "below") below++;
          else if (pos === "above") above++;
        }
        continue;
      }
      if (it.heading) {
        const full = clean(el.innerText);
        const txt = trunc(full, 80);
        const lvl = /^H([1-3])$/.exec(el.tagName);
        const hashes = "#".repeat(lvl ? Number(lvl[1]) : Math.min(3, Number(el.getAttribute("aria-level")) || 2));
        if (txt && !filter && !seen.has(txt)) {
          seen.add(txt);
          lines.push({ el, text: hashes + " " + txt, raw: full.toLowerCase(), heading: true });
        }
        continue;
      }
      if (it.media && el.tagName === "VIDEO" && (r.width < 120 || r.height < 60)) continue;
      if (it.media && el.tagName === "IMG" && (r.width < 48 || r.height < 48)) continue;
      if (inView && !av && hitMs < HIT_BUDGET_MS && mayBeCovered(el, ax, ay, r.width, r.height, layers)) {
        const t0 = performance.now();
        const hit = covered(el, r);
        hitMs += performance.now() - t0;
        if (hit) continue;
      }
      const role = roleOf(el);
      const name = nameOf(el);
      if (role === "clickable" && !name) continue;
      if (role === "link" && /^\[[\w\s]{1,5}\]$/.test(name) && (el.getAttribute("href") || "").startsWith("#")) continue;
      const ft = filter ? tierOf([name.toLowerCase(), role, String(el.value || "").toLowerCase()]) : null;
      if (filter && !ft.some((x) => x)) continue;
      const entry = { el, role, name, ft };
      if (collapseOk) {
        let lm = el.closest(LANDMARK);
        if (lm && (lm.tagName === "HEADER" || lm.getAttribute("role") === "banner") && lm.parentElement && lm.parentElement.closest("article,aside,main,nav,section")) lm = null;
        if (lm) {
          const box = land.get(lm) || { count: 0, entries: [] };
          box.count++;
          box.entries.push(entry);
          land.set(lm, box);
          entry.landmark = lm;
        }
        const item = el.closest(ITEM);
        if (item && item.parentElement) {
          const key = item.parentElement;
          const g = groups.get(key) || { items: new Set() };
          g.items.add(item);
          groups.set(key, g);
          entry.item = item;
          entry.list = key;
        }
      }
      lines.push(entry);
    }
    if (filter) {
      const best = filter.map((_, i) => lines.reduce((m, e) => Math.max(m, e.ft ? e.ft[i] : 0), 0));
      for (const e of lines) if (e.ft && !e.ft.some((x, i) => x && x === best[i])) e.drop = true;
    }
    const hrefOf = (e) => (e && !e.heading && e.role === "link" ? e.el.href || "" : null);
    for (let i = 0; i < lines.length; i++) {
      const e = lines[i], nx = lines[i + 1], pv = lines[i - 1];
      if (e.heading) {
        if (nx && !nx.heading && nx.name && (e.el.contains(nx.el) || nx.el.contains(e.el)) && nx.name.toLowerCase().startsWith(e.raw)) e.drop = true;
        continue;
      }
      const h = hrefOf(e);
      if (!h) continue;
      if (!e.name) {
        for (let j = Math.max(0, i - 3); j <= Math.min(lines.length - 1, i + 3); j++) {
          if (j !== i && hrefOf(lines[j]) === h && lines[j].name) e.drop = true;
        }
      } else if (hrefOf(pv) === h && !pv.drop && pv.name === e.name) e.drop = true;
    }
    const hit = filter ? filter.map((_, i) => lines.some((e) => !e.drop && !e.heading && e.ft && e.ft[i] > 0)) : [];
    const banner = (lm) => lm.tagName === "HEADER" || lm.getAttribute("role") === "banner";
    const bigLand = new Set([...land.entries()].filter(([lm, b]) => b.count > (banner(lm) ? 7 : 12)).map(([lm]) => lm));
    const bigList = new Map();
    for (const [list, g] of groups) if (g.items.size > 8) bigList.set(list, [...g.items].slice(0, 5));
    const out = [], landDone = new Set(), listDone = new Set();
    const ids = new Set();
    for (const e of lines) {
      if (e.drop) continue;
      if (e.heading) {
        out.push(e.text);
        continue;
      }
      if (e.landmark && bigLand.has(e.landmark)) {
        if (!landDone.has(e.landmark)) {
          landDone.add(e.landmark);
          const lm = e.landmark;
          const kind = lm.tagName === "NAV" || lm.getAttribute("role") === "navigation" ? "nav" : lm.tagName === "FOOTER" || lm.getAttribute("role") === "contentinfo" ? "footer" : lm.tagName === "HEADER" || lm.getAttribute("role") === "banner" ? "header" : "aside";
          const nm = trunc(clean(lm.getAttribute("aria-label") || ""), 30);
          ids.add(ref(lm));
          let names = "";
          if (kind === "header") {
            for (const x of land.get(lm).entries) {
              const n = trunc(x.name, 22);
              if (!n) continue;
              if (names.length + n.length > 150) {
                names += " | …";
                break;
              }
              names += (names ? " | " : ": ") + n;
            }
          }
          out.push(ref(lm) + " " + kind + (nm ? ' "' + q(nm) + '"' : "") + ": " + land.get(lm).count + " items (wisply snap -s " + ref(lm) + ")" + names);
        }
        continue;
      }
      if (e.list && bigList.has(e.list)) {
        const keep = bigList.get(e.list);
        if (!keep.includes(e.item)) {
          if (!listDone.has(e.list)) {
            listDone.add(e.list);
            const total = groups.get(e.list).items.size;
            ids.add(ref(e.list));
            out.push("... +" + (total - keep.length) + " similar items (wisply snap -s " + ref(e.list) + " --all)");
          }
          continue;
        }
      }
      ids.add(ref(e.el));
      out.push(line(e.el, e.role, e.name));
    }
    return { lines: out, ids, hit, modal: !!modal, late: !!cut.late, below: below + cut.below, above: above + cut.above, notes: notesOut.concat(notes(frames)) };
  }

  function textHits(terms) {
    const words = terms.map((w) => w.toLowerCase());
    const said = [], seen = new Set();
    for (const raw of (document.body ? document.body.innerText : "").split("\n")) {
      const s = raw.replace(/\s+/g, " ").trim();
      const low = s.toLowerCase();
      const pos = words.map((w) => low.indexOf(w));
      const fresh = pos.some((x, i) => x >= 0 && !seen.has(i));
      if (!fresh && (said.length >= 3 || !pos.some((x) => x >= 0))) continue;
      pos.forEach((x, i) => x >= 0 && seen.add(i));
      const at = Math.min(...pos.filter((x) => x >= 0));
      said.push('"' + (at > 60 ? "..." + trunc(s.slice(at - 30), 90) : trunc(s, 120)) + '"');
      if (said.length >= 3 && seen.size === words.length) break;
    }
    return { said, has: terms.filter((_, i) => seen.has(i)), none: terms.filter((_, i) => !seen.has(i)) };
  }

  function widen(opts, res) {
    const terms = opts.filter.split("|").map((s) => s.trim()).filter(Boolean);
    let miss = terms.filter((_, i) => !res.hit[i]);
    if (!miss.length) return res;
    if (!res.lines.some((l) => /^e\d+ /.test(l))) {
      if (!opts.all) res = collectLines(Object.assign({}, opts, { all: true }));
      miss = terms.filter((_, i) => !res.hit[i]);
      if (miss.length < terms.length) res.notes = res.notes.concat("(no match in view, matches from the whole page; click scrolls to them)");
    } else if (!opts.all && !res.modal) {
      const wide = collectLines(Object.assign({}, opts, { all: true, filter: miss.join("|"), until: performance.now() + WIDE_BUDGET_MS }));
      const found = wide.lines.filter((l) => /^e\d+ /.test(l) && !res.ids.has(l.split(" ")[0]));
      const fresh = found.slice(0, WIDE_LINES);
      for (const l of fresh) res.ids.add(l.split(" ")[0]);
      res.lines = res.lines.concat(fresh);
      const rest = found.length > fresh.length ? ", " + (found.length - fresh.length) + ' more: wisply snap --all -f "' + miss.join("|") + '"' : "";
      if (fresh.length > 1) res.notes = res.notes.concat("(the last " + fresh.length + " are outside the view; click scrolls to them" + rest + ")");
      else if (fresh.length) res.notes = res.notes.concat("(the last one is outside the view; click scrolls to it)");
      if (wide.late) {
        res.notes = res.notes.concat('(large page: stopped looking for "' + miss.join("|") + '" outside the view -> wisply snap --all -f "' + miss.join("|") + '")');
        miss = [];
      } else miss = miss.filter((_, i) => !wide.hit[i]);
    }
    if (miss.length) {
      const t = textHits(miss);
      if (t.has.length) res.notes = res.notes.concat(['(no control matches "' + t.has.join("|") + '", this text does:)'].concat(t.said));
      if (t.none.length) res.notes = res.notes.concat('(no match for "' + t.none.join("|") + '" on the whole page)');
    }
    res.searched = true;
    return res;
  }

  function snap(opts) {
    opts = opts || {};
    const sig = JSON.stringify([opts.all, opts.scope, opts.filter, opts.media, !!opts.links]);
    if (!opts.force && S.muts === S.lastMuts && location.href === S.lastHref && Math.round(scrollY) === S.lastY && sig === S.sig) {
      return { unchanged: true, title: document.title, url: location.href };
    }
    S.links = !!opts.links;
    let res = collectLines(opts);
    if (res.error) {
      S.links = false;
      return res;
    }
    if (opts.filter) res = widen(opts, res);
    S.links = false;
    const prev = S.last;
    S.last = res.ids;
    S.lastMuts = S.muts;
    S.lastHref = location.href;
    S.lastY = Math.round(scrollY);
    S.sig = sig;
    const out = { title: document.title, url: location.href, lines: res.lines, notes: res.notes, searched: res.searched, below: res.below, above: res.above };
    const seenLines = S.plainHref === location.href ? S.plainLines : null;
    if (opts.plain || opts.force) {
      S.plainHref = location.href;
      S.plainLines = new Set(res.lines);
    }
    if (opts.plain && seenLines) {
      const added = res.lines.filter((l) => !seenLines.has(l));
      const gone = [...seenLines].filter((l) => !S.plainLines.has(l)).length;
      if (added.length + gone <= Math.max(4, res.lines.length / 2)) {
        out.lines = added;
        out.delta = { gone, total: res.lines.length, chars: res.lines.reduce((n, l) => n + l.length + 1, 0) };
      }
    }
    if (S.keys.size) out.keys = [...S.keys].slice(0, 40);
    if (opts.diff) {
      out.lines = res.lines.filter((l) => /^e\d+ /.test(l) && !prev.has(l.split(" ")[0]));
      out.removed = [...prev].filter((r) => !res.ids.has(r)).length;
    }
    const sh = document.documentElement.scrollHeight;
    if (sh > innerHeight * 1.2) out.scroll = Math.round((scrollY / Math.max(1, sh - innerHeight)) * 100);
    return out;
  }

  function diff() {
    const res = collectLines({});
    const added = res.lines.filter((l) => /^e\d+ /.test(l) && !S.last.has(l.split(" ")[0]));
    const removed = [...S.last].filter((r) => !res.ids.has(r)).length;
    S.last = res.ids;
    S.lastMuts = -1;
    return { added, removed, total: res.ids.size };
  }

  Object.assign(L, { snap, diff });
})();
