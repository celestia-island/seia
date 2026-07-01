(function () {
  'use strict';

  var LANGUAGES = [
    { code: 'en', label: 'English', prefix: '', browserLangs: ['en'] },
    { code: 'zhs', label: '简体中文', prefix: '/zhs', browserLangs: ['zh-cn', 'zh-sg', 'zh-hans', 'zh'] }
  ];

  var NOTIFICATION_DISMISSED_KEY = 'seia-lang-notification-dismissed';

  var NOTIFICATION_TEXTS = {
    'zhs': { message: '此文档也有简体中文版本。', action: '切换到中文', dismiss: '不再提示' }
  };

  function detectCurrentLang() {
    var path = window.location.pathname;
    for (var i = 0; i < LANGUAGES.length; i++) {
      var prefix = LANGUAGES[i].prefix;
      if (prefix && (path.startsWith(prefix + '/') || path === prefix)) {
        return LANGUAGES[i];
      }
    }
    return LANGUAGES[0];
  }

  function getPagePath(currentLang) {
    var path = window.location.pathname;
    if (!currentLang.prefix) return path;
    return path.substring(currentLang.prefix.length) || '/';
  }

  function detectPreferredLang() {
    var browserLangs = (navigator.languages || [navigator.language || '']).map(function (l) {
      return l.toLowerCase();
    });
    for (var b = 0; b < browserLangs.length; b++) {
      var bl = browserLangs[b];
      for (var i = 0; i < LANGUAGES.length; i++) {
        for (var j = 0; j < LANGUAGES[i].browserLangs.length; j++) {
          if (bl === LANGUAGES[i].browserLangs[j] || bl.startsWith(LANGUAGES[i].browserLangs[j] + '-')) {
            return LANGUAGES[i];
          }
        }
      }
    }
    return null;
  }

  function showNotification(currentLang, suggestedLang) {
    var dismissed = null;
    try { dismissed = localStorage.getItem(NOTIFICATION_DISMISSED_KEY); } catch (e) { /* ignored */ }
    if (dismissed === suggestedLang.code) return;

    var texts = NOTIFICATION_TEXTS[suggestedLang.code];
    if (!texts) return;

    var pagePath = getPagePath(currentLang);

    var bar = document.createElement('div');
    bar.className = 'lang-notification';

    var msg = document.createElement('span');
    msg.className = 'lang-notification-text';
    msg.textContent = texts.message;

    var switchBtn = document.createElement('a');
    switchBtn.className = 'lang-notification-switch';
    switchBtn.href = suggestedLang.prefix + pagePath + window.location.hash;
    switchBtn.textContent = texts.action;

    var dismissBtn = document.createElement('button');
    dismissBtn.className = 'lang-notification-dismiss';
    dismissBtn.textContent = texts.dismiss;
    dismissBtn.addEventListener('click', function () {
      bar.classList.add('lang-notification-hide');
      try { localStorage.setItem(NOTIFICATION_DISMISSED_KEY, suggestedLang.code); } catch (e) { /* ignored */ }
      setTimeout(function () { bar.remove(); }, 300);
    });

    bar.appendChild(msg);
    bar.appendChild(switchBtn);
    bar.appendChild(dismissBtn);

    var tryInsertNotification = function () {
      var main = document.querySelector('#content') || document.querySelector('main') || document.body;
      main.insertBefore(bar, main.firstChild);
    };

    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', tryInsertNotification);
    } else {
      tryInsertNotification();
    }
  }

  function buildSwitcher() {
    var current = detectCurrentLang();
    var pagePath = getPagePath(current);

    var container = document.createElement('div');
    container.className = 'lang-switcher';

    var btn = document.createElement('button');
    btn.className = 'lang-switcher-btn';
    btn.setAttribute('aria-label', 'Switch language');
    btn.setAttribute('aria-expanded', 'false');

    var labelSpan = document.createElement('span');
    labelSpan.className = 'lang-label';
    var icon = document.createElement('span');
    icon.className = 'lang-icon';
    icon.textContent = '\uD83C\uDF10';
    labelSpan.appendChild(icon);
    labelSpan.appendChild(document.createTextNode(current.label));

    var arrow = document.createElement('span');
    arrow.className = 'lang-arrow';
    arrow.textContent = '\u25B2';

    btn.appendChild(labelSpan);
    btn.appendChild(arrow);

    var dropdown = document.createElement('div');
    dropdown.className = 'lang-switcher-dropdown';
    dropdown.setAttribute('role', 'listbox');

    LANGUAGES.forEach(function (lang) {
      var a = document.createElement('a');
      a.href = lang.prefix + pagePath + window.location.hash;
      a.textContent = lang.label;
      a.setAttribute('role', 'option');
      if (lang.code === current.code) {
        a.className = 'active';
        a.setAttribute('aria-selected', 'true');
      }
      dropdown.appendChild(a);
    });

    btn.addEventListener('click', function (e) {
      e.stopPropagation();
      var open = dropdown.classList.toggle('show');
      btn.setAttribute('aria-expanded', open ? 'true' : 'false');
    });

    document.addEventListener('click', function () {
      dropdown.classList.remove('show');
      btn.setAttribute('aria-expanded', 'false');
    });

    btn.addEventListener('keydown', function (e) {
      if (e.key === 'Escape') {
        dropdown.classList.remove('show');
        btn.setAttribute('aria-expanded', 'false');
      }
    });

    container.appendChild(dropdown);
    container.appendChild(btn);

    var tryInsert = function () {
      document.body.appendChild(container);
    };

    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', tryInsert);
    } else {
      tryInsert();
    }

    var preferred = detectPreferredLang();
    if (preferred && preferred.code !== current.code) {
      showNotification(current, preferred);
    }
  }

  buildSwitcher();
})();
