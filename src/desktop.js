// Runs inside the site before its own scripts, every page load (desktop app only).
// The site itself is untouched: same map, same features, live updates straight from gurista.co.uk.
(function () {
  'use strict';
  var VERSION = '__VERSION__';
  window.__SOVMAP_DESKTOP__ = { version: VERSION, chatlogs: true };
  window.__chatQ = [];
  // The app is already installed: never show the "install / download" prompts.
  try { localStorage.setItem('sovmap-install-snoozed', String(Date.now() + 3650 * 864e5)); } catch (e) {}
  var send = function (m) { try { window.ipc.postMessage(m); } catch (e) {} };

  var css = [
    'html.is-desktop .install-btn,html.is-desktop [data-pwa-install],html.is-desktop .sx{display:none!important}',
    'html.is-desktop ::-webkit-scrollbar{width:10px;height:10px}',
    'html.is-desktop ::-webkit-scrollbar-thumb{background:rgba(150,180,200,.22);border:3px solid transparent;background-clip:padding-box}',
    'html.is-desktop ::-webkit-scrollbar-thumb:hover{background-color:rgba(95,196,224,.5)}',
    'html.is-desktop ::-webkit-scrollbar-track{background:transparent}',
    'html.is-desktop ::selection{background:rgba(224,176,74,.35);color:#fff}',
    '.gd-pill{position:fixed;right:18px;top:84px;z-index:2147483000;display:flex;align-items:center;gap:12px;padding:12px 14px 12px 16px;',
    ' background:rgba(10,14,19,.94);border:1px solid #e0b04a;box-shadow:0 18px 50px rgba(0,0,0,.6),0 0 24px rgba(224,176,74,.18);',
    ' font:12px "IBM Plex Mono",ui-monospace,monospace;color:#f3d9a4;letter-spacing:.06em;animation:gd-in .35s cubic-bezier(.2,.9,.25,1)}',
    '.gd-pill b{display:block;font:600 14px "IBM Plex Sans Condensed",system-ui,sans-serif;letter-spacing:.02em;color:#f2f6f9}',
    '.gd-pill button{cursor:pointer;min-height:34px;padding:0 14px;background:rgba(224,176,74,.18);border:1px solid #e0b04a;color:#f3d9a4;',
    ' font:12px "IBM Plex Mono",ui-monospace,monospace;letter-spacing:.14em;text-transform:uppercase}',
    '.gd-pill button:hover{background:rgba(224,176,74,.32);color:#fff}',
    '.gd-pill .x{background:none;border:0;color:#7d8e9d;padding:0 4px;letter-spacing:0}',
    '.gd-bar{height:3px;width:160px;background:rgba(95,196,224,.15)}.gd-bar i{display:block;height:3px;background:#5fc4e0;width:0;transition:width .2s}',
    '@keyframes gd-in{from{opacity:0;transform:translateY(10px)}to{opacity:1;transform:none}}'
  ].join('\n');

  var pill = null;
  function el(html) { var d = document.createElement('div'); d.className = 'gd-pill'; d.innerHTML = html; document.body.appendChild(d); return d; }
  function esc(s) { return String(s).replace(/[&<>"]/g, function (c) { return { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]; }); }

  window.SovMapDesktop = {
    version: VERSION,
    update: function (r) {
      if (pill) pill.remove();
      var mb = r.size ? ' · ' + (r.size / 1048576).toFixed(1) + ' MB' : '';
      pill = el('<div><b>Version ' + esc(r.version) + ' is ready</b>' + (r.notes ? esc(r.notes).slice(0, 90) + (r.notes.length > 90 ? '…' : '') : 'Restarts in a couple of seconds') + mb + '</div>' +
        '<button type="button" data-u>Update</button><button type="button" class="x" data-x aria-label="Later">✕</button>');
      pill.querySelector('[data-u]').onclick = function () { send('update'); window.SovMapDesktop.progress(0); };
      pill.querySelector('[data-x]').onclick = function () { pill.remove(); pill = null; };
    },
    progress: function (p) {
      if (!pill) pill = el('');
      pill.innerHTML = '<div><b>Updating…</b><div class="gd-bar"><i style="width:' + Math.round(p * 100) + '%"></i></div></div>';
    },
    // Intel channels: new chat-log text from the app (see modules/channels.js on the site).
    chat: function (batch) {
      if (window.__chatQ) window.__chatQ.push.apply(window.__chatQ, batch);
      try { window.dispatchEvent(new CustomEvent('sovmap-chat', { detail: batch })); } catch (e) {}
    },
    failed: function (msg) {
      if (!pill) pill = el('');
      pill.innerHTML = '<div><b>Update didn\'t finish</b>' + esc(msg).slice(0, 80) + '</div><button type="button" data-w>Website</button><button type="button" class="x" data-x>✕</button>';
      pill.querySelector('[data-w]').onclick = function () { send('open:https://gurista.co.uk/sov-map/download.html'); };
      pill.querySelector('[data-x]').onclick = function () { pill.remove(); pill = null; };
    }
  };

  function ready() {
    document.documentElement.classList.add('is-desktop');
    var st = document.createElement('style'); st.textContent = css; document.head.appendChild(st);
    send('ready');
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', ready); else ready();

  // Links that leave the Sov Map (the gurista.co.uk logo, zKillboard, Dotlan...) open in the browser.
  document.addEventListener('click', function (e) {
    var a = e.target.closest && e.target.closest('a[href]');
    if (!a || e.defaultPrevented) return;
    var u = a.href;
    if (!/^https?:/i.test(u) || u.indexOf('https://gurista.co.uk/sov-map') === 0) return;
    e.preventDefault();
    send('open:' + u);
  }, true);

  document.addEventListener('keydown', function (e) {
    if (e.key === 'F11') { e.preventDefault(); send('fullscreen'); }
    if (e.key === 'F5' || (e.ctrlKey && e.key.toLowerCase() === 'r')) { e.preventDefault(); location.reload(); }
  }, true);
})();
