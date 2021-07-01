extern "C" {

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_staticMetaObject() -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_metaObject(
        this_ptr: *const crate::QUiLoader,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_qt_metacast(
        this_ptr: *mut crate::QUiLoader,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_qt_metacall(
        this_ptr: *mut crate::QUiLoader,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    /// <p>Creates a form loader with the given <i>parent</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#QUiLoader">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a form loader with the given <i>parent</i>.</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_QUiLoader(
        parent: *mut ::qt_core::QObject,
    ) -> *mut crate::QUiLoader;

    /// <p>Destroys the loader.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#dtor.QUiLoader">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Destroys the loader.</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_dQUiLoader(this_ptr: *mut crate::QUiLoader);

    /// <p>Returns a list naming the paths in which the loader will search when locating custom widget plugins.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#pluginPaths">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a list naming the paths in which the loader will search when locating custom widget plugins.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#addPluginPath">addPluginPath</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#clearPluginPaths">clearPluginPaths</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_pluginPaths(
        this_ptr: *const crate::QUiLoader,
    ) -> *mut ::qt_core::QStringList;

    /// <p>Clears the list of paths in which the loader will search when locating plugins.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#clearPluginPaths">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Clears the list of paths in which the loader will search when locating plugins.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#addPluginPath">addPluginPath</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#pluginPaths">pluginPaths</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_clearPluginPaths(this_ptr: *mut crate::QUiLoader);

    /// <p>Adds the given <i>path</i> to the list of paths in which the loader will search when locating plugins.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#addPluginPath">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Adds the given <i>path</i> to the list of paths in which the loader will search when locating plugins.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#pluginPaths">pluginPaths</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#clearPluginPaths">clearPluginPaths</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_addPluginPath(
        this_ptr: *mut crate::QUiLoader,
        path: *const ::qt_core::QString,
    );

    /// <p>Loads a form from the given <i>device</i> and creates a new widget with the given <i>parentWidget</i> to hold its contents.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#load">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Loads a form from the given <i>device</i> and creates a new widget with the given <i>parentWidget</i> to hold its contents.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#errorString">errorString</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_load(
        this_ptr: *mut crate::QUiLoader,
        device: *mut ::qt_core::QIODevice,
        parentWidget: *mut ::qt_widgets::QWidget,
    ) -> *mut ::qt_widgets::QWidget;

    /// <p>Returns a list naming all available widgets that can be built using the <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() function, i.e all the widgets specified within the given plugin paths.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a list naming all available widgets that can be built using the <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() function, i.e all the widgets specified within the given plugin paths.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#pluginPaths">pluginPaths</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_availableWidgets(
        this_ptr: *const crate::QUiLoader,
    ) -> *mut ::qt_core::QStringList;

    /// <p>Returns a list naming all available layouts that can be built using the <a href="http://doc.qt.io/qt-5/quiloader.html#createLayout">createLayout</a>() function</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#availableLayouts">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a list naming all available layouts that can be built using the <a href="http://doc.qt.io/qt-5/quiloader.html#createLayout">createLayout</a>() function</p>
    /// <p>This function was introduced in  Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createLayout">createLayout</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_availableLayouts(
        this_ptr: *const crate::QUiLoader,
    ) -> *mut ::qt_core::QStringList;

    /// <p>Creates a new widget with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>. You can use this function to create any of the widgets returned by the <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new widget with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>. You can use this function to create any of the widgets returned by the <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() function.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createWidget(
        this_ptr: *mut crate::QUiLoader,
        className: *const ::qt_core::QString,
        parent: *mut ::qt_widgets::QWidget,
        name: *const ::qt_core::QString,
    ) -> *mut ::qt_widgets::QWidget;

    /// <p>Creates a new layout with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createLayout">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new layout with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createLayout(
        this_ptr: *mut crate::QUiLoader,
        className: *const ::qt_core::QString,
        parent: *mut ::qt_core::QObject,
        name: *const ::qt_core::QString,
    ) -> *mut ::qt_widgets::QLayout;

    /// <p>Creates a new action group with the given <i>parent</i> and <i>name</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createActionGroup">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new action group with the given <i>parent</i> and <i>name</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createAction">createAction</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>(), and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createActionGroup(
        this_ptr: *mut crate::QUiLoader,
        parent: *mut ::qt_core::QObject,
        name: *const ::qt_core::QString,
    ) -> *mut ::qt_widgets::QActionGroup;

    /// <p>Creates a new action with the given <i>parent</i> and <i>name</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createAction">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new action with the given <i>parent</i> and <i>name</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createActionGroup">createActionGroup</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>(), and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createAction(
        this_ptr: *mut crate::QUiLoader,
        parent: *mut ::qt_core::QObject,
        name: *const ::qt_core::QString,
    ) -> *mut ::qt_widgets::QAction;

    /// <p>Sets the working directory of the loader to <i>dir</i>. The loader will look for other resources, such as icons and resource files, in paths relative to this directory.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#setWorkingDirectory">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Sets the working directory of the loader to <i>dir</i>. The loader will look for other resources, such as icons and resource files, in paths relative to this directory.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#workingDirectory">workingDirectory</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_setWorkingDirectory(
        this_ptr: *mut crate::QUiLoader,
        dir: *const ::qt_core::QDir,
    );

    /// <p>Returns the working directory of the loader.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#workingDirectory">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the working directory of the loader.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#setWorkingDirectory">setWorkingDirectory</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_workingDirectory(
        this_ptr: *const crate::QUiLoader,
    ) -> *mut ::qt_core::QDir;

    /// <p>If <i>enabled</i> is true, user interfaces loaded by this loader will automatically retranslate themselves upon receiving a language change event. Otherwise, the user interfaces will not be retranslated.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#setLanguageChangeEnabled">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>If <i>enabled</i> is true, user interfaces loaded by this loader will automatically retranslate themselves upon receiving a language change event. Otherwise, the user interfaces will not be retranslated.</p>
    /// <p>This function was introduced in  Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#isLanguageChangeEnabled">isLanguageChangeEnabled</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_setLanguageChangeEnabled(
        this_ptr: *mut crate::QUiLoader,
        enabled: bool,
    );

    /// <p>Returns true if dynamic retranslation on language change is enabled; returns false otherwise.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#isLanguageChangeEnabled">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns true if dynamic retranslation on language change is enabled; returns false otherwise.</p>
    /// <p>This function was introduced in  Qt 4.5.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#setLanguageChangeEnabled">setLanguageChangeEnabled</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_isLanguageChangeEnabled(
        this_ptr: *const crate::QUiLoader,
    ) -> bool;

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_setTranslationEnabled(
        this_ptr: *mut crate::QUiLoader,
        enabled: bool,
    );

    pub fn ctr_qt_ui_tools_ffi_QUiLoader_isTranslationEnabled(
        this_ptr: *const crate::QUiLoader,
    ) -> bool;

    /// <p>Returns a human-readable description of the last error occurred in <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#errorString">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns a human-readable description of the last error occurred in <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p>
    /// <p>This function was introduced in  Qt 5.0.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_errorString(
        this_ptr: *const crate::QUiLoader,
    ) -> *mut ::qt_core::QString;

    /// <p>The <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class enables standalone applications to dynamically create user interfaces at run-time using the information stored in UI files or specified in plugin paths.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class enables standalone applications to dynamically create user interfaces at run-time using the information stored in UI files or specified in plugin paths.</p>
    /// <p>In addition, you can customize or create your own user interface by deriving your own loader class.</p>
    /// <p>If you have a custom component or an application that embeds <i>Qt Designer</i>, you can also use the <a href="http://doc.qt.io/qt-5/qformbuilder.html">QFormBuilder</a> class provided by the <a href="http://doc.qt.io/qt-5/qtdesigner-module.html">QtDesigner</a> module to create user interfaces from UI files.</p>
    /// <p>The <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class provides a collection of functions allowing you to create widgets based on the information stored in UI files (created with <i>Qt Designer</i>) or available in the specified plugin paths. The specified plugin paths can be retrieved using the <a href="http://doc.qt.io/qt-5/quiloader.html#pluginPaths">pluginPaths</a>() function. Similarly, the contents of a UI file can be retrieved using the <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>() function. For example:</p>
    /// <pre class="cpp">
    ///
    ///   MyWidget<span class="operator">::</span>MyWidget(<span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span>parent)
    /// &#32;     : <span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span>(parent)
    ///   {
    /// &#32;     <span class="type"><a href="http://doc.qt.io/qt-5/quiloader.html#QUiLoader">QUiLoader</a></span> loader;
    /// &#32;     <span class="type"><a href="http://doc.qt.io/qt-5/qfile.html">QFile</a></span> file(<span class="string">":/forms/myform.ui"</span>);
    /// &#32;     file<span class="operator">.</span>open(<span class="type"><a href="http://doc.qt.io/qt-5/qfile.html">QFile</a></span><span class="operator">::</span>ReadOnly);
    /// &#32;     <span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span>myWidget <span class="operator">=</span> loader<span class="operator">.</span>load(<span class="operator">&amp;</span>file<span class="operator">,</span> <span class="keyword">this</span>);
    /// &#32;     file<span class="operator">.</span>close();
    ///
    /// &#32;     <span class="type"><a href="http://doc.qt.io/qt-5/qvboxlayout.html">QVBoxLayout</a></span> <span class="operator">*</span>layout <span class="operator">=</span> <span class="keyword">new</span> <span class="type"><a href="http://doc.qt.io/qt-5/qvboxlayout.html">QVBoxLayout</a></span>;
    /// &#32;     layout<span class="operator">-</span><span class="operator">&gt;</span>addWidget(myWidget);
    /// &#32;     setLayout(layout);
    ///   }
    ///
    /// </pre>
    /// <p>By including the user interface in the form's resources (<code>myform.qrc</code>), we ensure that it will be present at run-time:</p>
    /// <pre class="cpp">
    ///
    ///   &lt;!DOCTYPE RCC&gt;&lt;RCC version="1.0"&gt;
    ///   &lt;qresource prefix="/forms"&gt;
    ///   &lt;file&gt;myform.ui&lt;/file&gt;
    ///   &lt;/qresource&gt;
    ///   &lt;/RCC&gt;
    ///
    /// </pre>
    /// <p>The <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() function returns a <a href="http://doc.qt.io/qt-5/qstringlist.html">QStringList</a> with the class names of the widgets available in the specified plugin paths. To create these widgets, simply use the <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() function. For example:</p>
    /// <pre class="cpp">
    ///
    ///   <span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span>loadCustomWidget(<span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span>parent)
    ///   {
    /// &#32;     <span class="type"><a href="http://doc.qt.io/qt-5/quiloader.html#QUiLoader">QUiLoader</a></span> loader;
    /// &#32;     <span class="type"><a href="http://doc.qt.io/qt-5/qwidget.html">QWidget</a></span> <span class="operator">*</span>myWidget;
    ///
    /// &#32;     <span class="type"><a href="http://doc.qt.io/qt-5/qstringlist.html">QStringList</a></span> availableWidgets <span class="operator">=</span> loader<span class="operator">.</span>availableWidgets();
    ///
    /// &#32;     <span class="keyword">if</span> (availableWidgets<span class="operator">.</span>contains(<span class="string">"AnalogClock"</span>))
    /// &#32;   &#32;     myWidget <span class="operator">=</span> loader<span class="operator">.</span>createWidget(<span class="string">"AnalogClock"</span><span class="operator">,</span> parent);
    ///
    /// &#32;     <span class="keyword">return</span> myWidget;
    ///   }
    ///
    /// </pre>
    /// <p>To make a custom widget available to the loader, you can use the <a href="http://doc.qt.io/qt-5/quiloader.html#addPluginPath">addPluginPath</a>() function; to remove all available widgets, you can call the <a href="http://doc.qt.io/qt-5/quiloader.html#clearPluginPaths">clearPluginPaths</a>() function.</p>
    /// <p>The <a href="http://doc.qt.io/qt-5/quiloader.html#createAction">createAction</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createActionGroup">createActionGroup</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createLayout">createLayout</a>(), and <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() functions are used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it has to create an action, action group, layout, or widget respectively. For that reason, you can subclass the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class and reimplement these functions to intervene the process of constructing a user interface. For example, you might want to have a list of the actions created when loading a form or creating a custom widget.</p>
    /// <p>For a complete example using the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class, see the <a href="http://doc.qt.io/qt-5/qtdesigner-calculatorbuilder-example.html">Calculator Builder Example</a>.</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_QUiLoader1() -> *mut crate::QUiLoader;

    /// <p>Loads a form from the given <i>device</i> and creates a new widget with the given <i>parentWidget</i> to hold its contents.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#load">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Loads a form from the given <i>device</i> and creates a new widget with the given <i>parentWidget</i> to hold its contents.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#errorString">errorString</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_load1(
        this_ptr: *mut crate::QUiLoader,
        device: *mut ::qt_core::QIODevice,
    ) -> *mut ::qt_widgets::QWidget;

    /// <p>Creates a new widget with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>. You can use this function to create any of the widgets returned by the <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new widget with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>. You can use this function to create any of the widgets returned by the <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() function.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createWidget1(
        this_ptr: *mut crate::QUiLoader,
        className: *const ::qt_core::QString,
        parent: *mut ::qt_widgets::QWidget,
    ) -> *mut ::qt_widgets::QWidget;

    /// <p>Creates a new widget with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>. You can use this function to create any of the widgets returned by the <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() function.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new widget with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>. You can use this function to create any of the widgets returned by the <a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() function.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#availableWidgets">availableWidgets</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createWidget2(
        this_ptr: *mut crate::QUiLoader,
        className: *const ::qt_core::QString,
    ) -> *mut ::qt_widgets::QWidget;

    /// <p>Creates a new layout with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createLayout">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new layout with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createLayout1(
        this_ptr: *mut crate::QUiLoader,
        className: *const ::qt_core::QString,
        parent: *mut ::qt_core::QObject,
    ) -> *mut ::qt_widgets::QLayout;

    /// <p>Creates a new layout with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createLayout">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new layout with the given <i>parent</i> and <i>name</i> using the class specified by <i>className</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>() and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createLayout2(
        this_ptr: *mut crate::QUiLoader,
        className: *const ::qt_core::QString,
    ) -> *mut ::qt_widgets::QLayout;

    /// <p>Creates a new action group with the given <i>parent</i> and <i>name</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createActionGroup">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new action group with the given <i>parent</i> and <i>name</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createAction">createAction</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>(), and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createActionGroup1(
        this_ptr: *mut crate::QUiLoader,
        parent: *mut ::qt_core::QObject,
    ) -> *mut ::qt_widgets::QActionGroup;

    /// <p>Creates a new action group with the given <i>parent</i> and <i>name</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createActionGroup">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new action group with the given <i>parent</i> and <i>name</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createAction">createAction</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>(), and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createActionGroup2(
        this_ptr: *mut crate::QUiLoader,
    ) -> *mut ::qt_widgets::QActionGroup;

    /// <p>Creates a new action with the given <i>parent</i> and <i>name</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createAction">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new action with the given <i>parent</i> and <i>name</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createActionGroup">createActionGroup</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>(), and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createAction1(
        this_ptr: *mut crate::QUiLoader,
        parent: *mut ::qt_core::QObject,
    ) -> *mut ::qt_widgets::QAction;

    /// <p>Creates a new action with the given <i>parent</i> and <i>name</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/quiloader.html#createAction">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Creates a new action with the given <i>parent</i> and <i>name</i>.</p>
    /// <p>The function is also used internally by the <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> class whenever it creates a widget. Hence, you can subclass <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a> and reimplement this function to intervene process of constructing a user interface or widget. However, in your implementation, ensure that you call <a href="http://doc.qt.io/qt-5/quiloader.html">QUiLoader</a>'s version first.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/quiloader.html#createActionGroup">createActionGroup</a>(), <a href="http://doc.qt.io/qt-5/quiloader.html#createWidget">createWidget</a>(), and <a href="http://doc.qt.io/qt-5/quiloader.html#load">load</a>().</p></div>
    pub fn ctr_qt_ui_tools_ffi_QUiLoader_createAction2(
        this_ptr: *mut crate::QUiLoader,
    ) -> *mut ::qt_widgets::QAction;

    pub fn ctr_qt_ui_tools_ffi_static_cast_QUiLoader_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QUiLoader;

    pub fn ctr_qt_ui_tools_ffi_static_cast_QObject_ptr(
        ptr: *mut crate::QUiLoader,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_ui_tools_ffi_dynamic_cast_QUiLoader_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QUiLoader;

}
