#include "qt_ui_tools_c_global.h"
extern "C" {
RITUAL_EXPORT QMetaObject const * ctr_qt_ui_tools_ffi_QUiLoader_staticMetaObject() {
  return &QUiLoader::staticMetaObject;
}


RITUAL_EXPORT QMetaObject const * ctr_qt_ui_tools_ffi_QUiLoader_metaObject(QUiLoader const * this_ptr) {
  return this_ptr->metaObject();
}


RITUAL_EXPORT void * ctr_qt_ui_tools_ffi_QUiLoader_qt_metacast(QUiLoader * this_ptr, char const * arg1) {
  return this_ptr->qt_metacast(arg1);
}


RITUAL_EXPORT int ctr_qt_ui_tools_ffi_QUiLoader_qt_metacall(QUiLoader * this_ptr, QMetaObject::Call arg1, int arg2, void * * arg3) {
  return this_ptr->qt_metacall(arg1, arg2, arg3);
}


RITUAL_EXPORT QString * ctr_qt_ui_tools_ffi_QUiLoader_tr(char const * s, char const * c, int n) {
  return new QString(QUiLoader::tr(s, c, n));
}


RITUAL_EXPORT QString * ctr_qt_ui_tools_ffi_QUiLoader_trUtf8(char const * s, char const * c, int n) {
  return new QString(QUiLoader::trUtf8(s, c, n));
}


RITUAL_EXPORT QUiLoader * ctr_qt_ui_tools_ffi_QUiLoader_QUiLoader(QObject * parent) {
  return new QUiLoader(parent);
}


RITUAL_EXPORT void ctr_qt_ui_tools_ffi_QUiLoader_dQUiLoader(QUiLoader * this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT QStringList * ctr_qt_ui_tools_ffi_QUiLoader_pluginPaths(QUiLoader const * this_ptr) {
  return new QStringList(this_ptr->pluginPaths());
}


RITUAL_EXPORT void ctr_qt_ui_tools_ffi_QUiLoader_clearPluginPaths(QUiLoader * this_ptr) {
  this_ptr->clearPluginPaths();
}


RITUAL_EXPORT void ctr_qt_ui_tools_ffi_QUiLoader_addPluginPath(QUiLoader * this_ptr, QString const * path) {
  this_ptr->addPluginPath(*path);
}


RITUAL_EXPORT QWidget * ctr_qt_ui_tools_ffi_QUiLoader_load(QUiLoader * this_ptr, QIODevice * device, QWidget * parentWidget) {
  return this_ptr->load(device, parentWidget);
}


RITUAL_EXPORT QStringList * ctr_qt_ui_tools_ffi_QUiLoader_availableWidgets(QUiLoader const * this_ptr) {
  return new QStringList(this_ptr->availableWidgets());
}


RITUAL_EXPORT QStringList * ctr_qt_ui_tools_ffi_QUiLoader_availableLayouts(QUiLoader const * this_ptr) {
  return new QStringList(this_ptr->availableLayouts());
}


RITUAL_EXPORT QWidget * ctr_qt_ui_tools_ffi_QUiLoader_createWidget(QUiLoader * this_ptr, QString const * className, QWidget * parent, QString const * name) {
  return this_ptr->createWidget(*className, parent, *name);
}


RITUAL_EXPORT QLayout * ctr_qt_ui_tools_ffi_QUiLoader_createLayout(QUiLoader * this_ptr, QString const * className, QObject * parent, QString const * name) {
  return this_ptr->createLayout(*className, parent, *name);
}


RITUAL_EXPORT QActionGroup * ctr_qt_ui_tools_ffi_QUiLoader_createActionGroup(QUiLoader * this_ptr, QObject * parent, QString const * name) {
  return this_ptr->createActionGroup(parent, *name);
}


RITUAL_EXPORT QAction * ctr_qt_ui_tools_ffi_QUiLoader_createAction(QUiLoader * this_ptr, QObject * parent, QString const * name) {
  return this_ptr->createAction(parent, *name);
}


RITUAL_EXPORT void ctr_qt_ui_tools_ffi_QUiLoader_setWorkingDirectory(QUiLoader * this_ptr, QDir const * dir) {
  this_ptr->setWorkingDirectory(*dir);
}


RITUAL_EXPORT QDir * ctr_qt_ui_tools_ffi_QUiLoader_workingDirectory(QUiLoader const * this_ptr) {
  return new QDir(this_ptr->workingDirectory());
}


RITUAL_EXPORT void ctr_qt_ui_tools_ffi_QUiLoader_setLanguageChangeEnabled(QUiLoader * this_ptr, bool enabled) {
  this_ptr->setLanguageChangeEnabled(enabled);
}


RITUAL_EXPORT bool ctr_qt_ui_tools_ffi_QUiLoader_isLanguageChangeEnabled(QUiLoader const * this_ptr) {
  return this_ptr->isLanguageChangeEnabled();
}


RITUAL_EXPORT void ctr_qt_ui_tools_ffi_QUiLoader_setTranslationEnabled(QUiLoader * this_ptr, bool enabled) {
  this_ptr->setTranslationEnabled(enabled);
}


RITUAL_EXPORT bool ctr_qt_ui_tools_ffi_QUiLoader_isTranslationEnabled(QUiLoader const * this_ptr) {
  return this_ptr->isTranslationEnabled();
}


RITUAL_EXPORT QString * ctr_qt_ui_tools_ffi_QUiLoader_errorString(QUiLoader const * this_ptr) {
  return new QString(this_ptr->errorString());
}


RITUAL_EXPORT QUiLoader * ctr_qt_ui_tools_ffi_QUiLoader_QUiLoader1() {
  return new QUiLoader();
}


RITUAL_EXPORT QWidget * ctr_qt_ui_tools_ffi_QUiLoader_load1(QUiLoader * this_ptr, QIODevice * device) {
  return this_ptr->load(device);
}


RITUAL_EXPORT QWidget * ctr_qt_ui_tools_ffi_QUiLoader_createWidget1(QUiLoader * this_ptr, QString const * className, QWidget * parent) {
  return this_ptr->createWidget(*className, parent);
}


RITUAL_EXPORT QWidget * ctr_qt_ui_tools_ffi_QUiLoader_createWidget2(QUiLoader * this_ptr, QString const * className) {
  return this_ptr->createWidget(*className);
}


RITUAL_EXPORT QLayout * ctr_qt_ui_tools_ffi_QUiLoader_createLayout1(QUiLoader * this_ptr, QString const * className, QObject * parent) {
  return this_ptr->createLayout(*className, parent);
}


RITUAL_EXPORT QLayout * ctr_qt_ui_tools_ffi_QUiLoader_createLayout2(QUiLoader * this_ptr, QString const * className) {
  return this_ptr->createLayout(*className);
}


RITUAL_EXPORT QActionGroup * ctr_qt_ui_tools_ffi_QUiLoader_createActionGroup1(QUiLoader * this_ptr, QObject * parent) {
  return this_ptr->createActionGroup(parent);
}


RITUAL_EXPORT QActionGroup * ctr_qt_ui_tools_ffi_QUiLoader_createActionGroup2(QUiLoader * this_ptr) {
  return this_ptr->createActionGroup();
}


RITUAL_EXPORT QAction * ctr_qt_ui_tools_ffi_QUiLoader_createAction1(QUiLoader * this_ptr, QObject * parent) {
  return this_ptr->createAction(parent);
}


RITUAL_EXPORT QAction * ctr_qt_ui_tools_ffi_QUiLoader_createAction2(QUiLoader * this_ptr) {
  return this_ptr->createAction();
}


RITUAL_EXPORT QUiLoader * ctr_qt_ui_tools_ffi_static_cast_QUiLoader_ptr(QObject * ptr) {
  return static_cast< QUiLoader * >(ptr);
}


RITUAL_EXPORT QObject * ctr_qt_ui_tools_ffi_static_cast_QObject_ptr(QUiLoader * ptr) {
  return static_cast< QObject * >(ptr);
}


RITUAL_EXPORT QUiLoader * ctr_qt_ui_tools_ffi_dynamic_cast_QUiLoader_ptr(QObject * ptr) {
  return dynamic_cast< QUiLoader * >(ptr);
}


} // extern "C"
