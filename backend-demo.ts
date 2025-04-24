type Paper = {
    id: number;
    title: string;
};

type Professor = {
    id: number;
    name: string;
    dept: string;
    desc: string;
    papers: Paper[];
};

async function getAllProfessors(addr: string): Promise<Professor[]> {
    const res = await fetch(`http://${addr}/api/professors`);
    return await res.json();
}

async function getProfessor(addr: string, id: number): Promise<Professor | null> {
    const res = await fetch(`http://${addr}/api/professors/${id}`);
    if (!res.ok) {
        const json = await res.json();
        console.log('Error getting Professor:', json);
        return null;
    }
    return await res.json();
}

async function getPapers(addr: string, profId: number): Promise<Paper[] | null> {
    const res = await fetch(`http://${addr}/api/professors/${profId}/papers`);
    if (!res.ok) {
        const json = await res.json();
        console.log('Error getting Papers:', json);
        return null;
    }
    return await res.json();
}

async function addProfessor(addr: string): Promise<Professor> {
    const res = await fetch(`http://${addr}/api/professors`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            name: 'Demo Professor',
            dept: 'Demo Department',
            desc: 'Some demo description about this professor'
        }),
    });
    return await res.json();
}

async function addPapers(addr: string, id: number): Promise<Paper[]> {
    const titles = [
        "Concurrency in Swift: Analyzing Structured Concurrency with async/await",
        "Swift Performance Optimization: Benchmarking Native Code and Compiler Improvements",
        "Leveraging Machine Learning in Swift: Integrating Core ML in Real-Time Applications",
        "Protocol-Oriented Programming in Swift: Redefining Code Reusability",
        "Type Safety and Optionals in Swift: Enhancing Code Reliability in Modern Applications"
    ];
    await Promise.all(titles.map(t => fetch(`http://${addr}/api/professors/${id}/papers`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ title: t })
    })));
    const res = await fetch(`http://${addr}/api/professors/${id}/papers`);
    return await res.json();
}

async function deleteProfessor(addr: string, id: number) {
    const res = await fetch(`http://${addr}/api/professors/${id}`, {
        method: 'DELETE'
    });
    if (!res.ok) {
        const json = await res.json();
        console.log('Error deleting Professor:', json);
    }
}

async function generateProfessorDesc(addr: string, id: number): Promise<{ description: string }> {
    const res = await fetch(`http://${addr}/api/professors/${id}/description`);
    return await res.json();
}

async function runDemo() {
    let addr = prompt('Please enter server address:');
    if (addr === null) return;
    addr = addr.replace('localhost', '127.0.0.1');

    section('All Professors');
    console.log('All Professors:', await getAllProfessors(addr), '\n');

    section('Professor 0');
    console.log('Professor 0:', await getProfessor(addr, 0), '\n');

    section('Some Non-Existent Professor (Prof. 420)');
    console.log('Nonexistent Professor:', await getProfessor(addr, 420), '\n');

    section('All Papers for Professor 0');
    console.log('Professor 0 Papers:', await getPapers(addr, 0), '\n');

    section('Add New Professor');
    const addedProfessor = await addProfessor(addr);
    console.log('Added Professor', addedProfessor, '\n');

    section('Add Papers to New Professor');
    console.log('Added Papers:', await addPapers(addr, addedProfessor.id));

    section('Generate New Professor Description');
    console.log('New Description:', await generateProfessorDesc(addr, addedProfessor.id));

    section('Added Professor State');
    console.log('Added Professor', await getProfessor(addr, addedProfessor.id), '\n');

    section('Delete New Professor');
    await deleteProfessor(addr, addedProfessor.id);

    section('Get Deleted Professor');
    console.log('Deleted Professor:', await getProfessor(addr, addedProfessor.id), '\n');
}

const section = (title: string) => {
    console.log('='.repeat(80));
    console.log(`\t${title}`);
    console.log('='.repeat(80));
}

runDemo();