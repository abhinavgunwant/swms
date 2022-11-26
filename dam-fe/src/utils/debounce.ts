/**
 * Returns a function, that, as long as it continues to be invoked, will not
 * be triggered. The function will be called after it stops being called for
 * `wait` milliseconds.
 * 
 * Originally inspired by  David Walsh (https://davidwalsh.name/javascript-debounce-function)
 * @see https://www.educative.io/edpresso/how-to-use-the-debounce-function-in-javascript (for this one!)
 * @see https://levelup.gitconnected.com/debounce-in-javascript-improve-your-applications-performance-5b01855e086
 * 
 * @param func callback function
 * @param wait delay
 */
function debounce (func: Function, wait: number, immediate: boolean) {
    let timeout: any;
  
    return function executedFunction(this: Function) {
        const context = this;
        const args = arguments;
            
        var later = function() {
            timeout = null;

            if (!immediate) {
                func.apply(context, args)
            };
        };

        var callNow = immediate && !timeout;
        
        clearTimeout(timeout);

        timeout = setTimeout(later, wait);
        
        if (callNow) {
            func.apply(context, args)
        };
    };
};

export default debounce;
