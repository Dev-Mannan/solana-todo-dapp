import { useEffect, useState } from 'react';
import Loading from '../components/Loading';
import TodoSection from '../components/todo/TodoSection';
import styles from '../styles/Home.module.css';
import dynamic from 'next/dynamic';
import { useTodo } from '../hooks/todo';



const DynamicWalletMultiButton = dynamic(
    () => import('@solana/wallet-adapter-react-ui').then(mod => mod.WalletMultiButton),
    { ssr: false }
);

const Home = () => {
    const [isClient, setIsClient] = useState(false);
    const {
        initialized,
        initializeUser,
        loading,
        transactionPending,
        completedTodos,
        incompleteTodos,
        addTodo,
        markTodo,
        removeTodo,
        // markStaticTodo,
        removeStaticTodo,
        addStaticTodo,
        input,
        handleChange
    } = useTodo();

    useEffect(() => {
        setIsClient(true);
    }, []);

    return (
        <div className={styles.container}>
            <div className={styles.actionsContainer}>
                {initialized ? (
                    <div className={styles.todoInput}>
                        <div className={`${styles.todoCheckbox} ${styles.checked}`} />
                        <div className={styles.inputContainer}>
                            
                            <form onSubmit={addTodo}>
                                <input value={input} onChange={handleChange} id={styles.inputField} type="text" placeholder='Create a new todo...' />
                            </form>
                        </div>
                        <div className={styles.iconContainer}></div>
                    </div>
                ) : (
                    <button type="button" className={styles.button} onClick={initializeUser} disabled={transactionPending}>
                        Initialize
                    </button>
                )}
                {isClient && <DynamicWalletMultiButton />}
            </div>

            <div className={styles.mainContainer}>
                <Loading loading={loading}>
                    <TodoSection title="Tasks" todos={incompleteTodos} action={markTodo} />
                    <TodoSection title="Completed" todos={completedTodos} action={removeTodo} />
                </Loading>
            </div>
        </div>
    );
};

export default Home;
